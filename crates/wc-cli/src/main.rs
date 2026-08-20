use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use std::io::Write;
use std::path::PathBuf;
use wc_ai::build_router;
use wc_core::config::{ensure_db_from_seed, load_config, save_config};
use wc_core::db::CommandStore;
use wc_core::models::{AiContext, AppSettings};

#[derive(Parser)]
#[command(name = "wc", about = "What Command — CLI helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search the command database
    Search {
        query: String,
        #[arg(short, long, default_value = "20")]
        limit: usize,
    },
    /// Ask AI to suggest a command
    Ask { prompt: String },
    /// Explain a command via AI
    Explain { command: String },
    /// Refresh bundled command database from seed
    Update,
    /// Configure AI providers, API keys, and models
    ///
    /// Run `wc settings` with no subcommand for a summary (same as `wc settings list`).
    Settings {
        #[command(subcommand)]
        command: Option<SettingsCmd>,
    },
    /// Print configuration paths
    Config {
        #[command(subcommand)]
        command: ConfigCmd,
    },
    /// Download or list on-device GGUF models
    Model {
        #[command(subcommand)]
        command: ModelCmd,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

#[derive(Subcommand)]
enum ConfigCmd {
    /// Print the path to config.toml
    Path,
    /// Print the config directory
    Dir,
}

#[derive(Subcommand)]
enum ModelCmd {
    /// Download a GGUF model into ~/.config/what-command/models/
    ///
    /// Defaults to the configured local_model_id (gemma-2b-it-q4).
    Download {
        /// Model id (e.g. gemma-2b-it-q4). Omit for the default.
        id: Option<String>,
        /// Skip if a file with matching size already exists
        #[arg(short, long)]
        skip_existing: bool,
    },
    /// List known models and any already downloaded to the models dir
    List,
}

#[derive(Subcommand)]
enum SettingsCmd {
    /// List current configuration and config file path
    List {
        /// Emit machine-readable JSON instead of a summary table
        #[arg(short, long)]
        json: bool,
        /// Include raw (unmasked) values for secret keys
        #[arg(short, long)]
        raw: bool,
    },
    /// Print the value of a single setting
    Show {
        key: String,
        /// Print raw (unmasked) values for secret keys
        #[arg(short, long)]
        raw: bool,
    },
    /// Set a single setting by key and persist it to config.toml
    Set { key: String, value: String },
    /// Open the config file in $EDITOR
    Edit,
    /// Reset config to defaults
    Reset,
    /// Print environment variables that override config
    Env,
}

fn db_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("what-command")
        .join("commands.db")
}

fn seed_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data/commands.db")
}

const VALID_KEYS: &[&str] = &[
    "ai_provider",
    "ai_model",
    "fallback_provider",
    "fallback_model",
    "opencode_api_key",
    "kilo_api_key",
    "local_model_id",
    "local_model_path",
    "local_max_tokens",
    "openai_compat_base_url",
    "openai_compat_api_key",
];

fn is_secret_key(key: &str) -> bool {
    matches!(
        key,
        "opencode_api_key" | "kilo_api_key" | "openai_compat_api_key"
    )
}

fn mask(value: &str) -> String {
    if value.len() <= 4 {
        "••••".into()
    } else {
        format!("••••:{}", &value[value.len() - 4..])
    }
}

/// Validate that `key` is a known setting, returning it back.
///
/// Callers must use this before [`get_key`] so that a valid-but-unset value
/// (which [`get_key`] reports as `None`) is never mistaken for an unknown key.
fn resolve_key(key: &str) -> Result<&str, String> {
    if VALID_KEYS.contains(&key) {
        Ok(key)
    } else {
        Err(format!(
            "unknown key '{key}'. Valid: {}",
            VALID_KEYS.join(", ")
        ))
    }
}

/// Human-readable display of a setting value, masking secrets unless `raw`.
/// Returns `"<none>"` for unset/empty values.
fn display_value(value: Option<&str>, key: &str, raw: bool) -> String {
    match value {
        Some(v) if !v.is_empty() => {
            if is_secret_key(key) && !raw {
                mask(v)
            } else {
                v.to_string()
            }
        }
        _ => "<none>".to_string(),
    }
}

fn get_key(settings: &AppSettings, key: &str) -> Option<String> {
    Some(match key {
        "ai_provider" => settings.ai_provider.clone(),
        "ai_model" => settings.ai_model.clone(),
        "fallback_provider" => return settings.fallback_provider.clone(),
        "fallback_model" => return settings.fallback_model.clone(),
        "opencode_api_key" => return settings.opencode_api_key.clone(),
        "kilo_api_key" => return settings.kilo_api_key.clone(),
        "local_model_id" => return settings.local_model_id.clone(),
        "local_model_path" => return settings.local_model_path.clone(),
        "local_max_tokens" => return settings.local_max_tokens.map(|n| n.to_string()),
        "openai_compat_base_url" => return settings.openai_compat_base_url.clone(),
        "openai_compat_api_key" => return settings.openai_compat_api_key.clone(),
        _ => return None,
    })
}

fn set_key(settings: &mut AppSettings, key: &str, value: String) -> Result<(), String> {
    match key {
        "ai_provider" => {
            if !is_valid_provider(&value) {
                return Err(format!(
                    "invalid ai_provider '{value}'. Valid: opencode_zen, kilo_gateway, local_llm, openai_compat"
                ));
            }
            settings.ai_provider = value;
        }
        "ai_model" => settings.ai_model = value,
        "fallback_provider" => {
            if !is_valid_provider(&value) {
                return Err(format!(
                    "invalid fallback_provider '{value}'. Valid: opencode_zen, kilo_gateway, local_llm, openai_compat"
                ));
            }
            settings.fallback_provider = Some(value);
        }
        "fallback_model" => settings.fallback_model = Some(value),
        "opencode_api_key" => settings.opencode_api_key = Some(value),
        "kilo_api_key" => settings.kilo_api_key = Some(value),
        "local_model_id" => settings.local_model_id = Some(value),
        "local_model_path" => settings.local_model_path = Some(value),
        "local_max_tokens" => {
            settings.local_max_tokens = Some(
                value
                    .parse()
                    .map_err(|e| format!("local_max_tokens must be a number: {e}"))?,
            )
        }
        "openai_compat_base_url" => settings.openai_compat_base_url = Some(value),
        "openai_compat_api_key" => settings.openai_compat_api_key = Some(value),
        _ => {
            return Err(format!(
                "unknown key '{key}'. Valid: {}",
                VALID_KEYS.join(", ")
            ))
        }
    }
    Ok(())
}

fn is_valid_provider(name: &str) -> bool {
    matches!(
        name,
        "opencode_zen" | "kilo_gateway" | "local_llm" | "openai_compat"
    )
}

fn clear_key(settings: &mut AppSettings, key: &str) -> Result<(), String> {
    match key {
        "ai_provider" => settings.ai_provider = String::new(),
        "ai_model" => settings.ai_model = String::new(),
        "fallback_provider" => settings.fallback_provider = None,
        "fallback_model" => settings.fallback_model = None,
        "opencode_api_key" => settings.opencode_api_key = None,
        "kilo_api_key" => settings.kilo_api_key = None,
        "local_model_id" => settings.local_model_id = None,
        "local_model_path" => settings.local_model_path = None,
        "local_max_tokens" => settings.local_max_tokens = None,
        "openai_compat_base_url" => settings.openai_compat_base_url = None,
        "openai_compat_api_key" => settings.openai_compat_api_key = None,
        _ => return Err(format!("unknown key '{key}'")),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let path = db_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    ensure_db_from_seed(&seed_path(), &path)?;
    let store = CommandStore::open(&path)?;
    store.init_schema()?;

    match cli.command {
        Commands::Search { query, limit } => {
            let hits = store.search(&query, limit)?;
            for cmd in hits {
                println!("{} — {}", cmd.command, cmd.description);
            }
        }
        Commands::Ask { prompt } => {
            let config = load_config()?;
            let router = build_router(&config.settings);
            let ctx = AiContext::default();
            match router.generate_command(&prompt, &ctx).await {
                Ok(s) => {
                    println!("{}\n# {}", s.command, s.explanation);
                }
                Err(e) => {
                    eprintln!("AI error: {e}");
                    let stub = wc_ai::router::stub_suggestion(&prompt);
                    println!("{}\n# {}", stub.command, stub.explanation);
                }
            }
        }
        Commands::Explain { command } => {
            let config = load_config()?;
            let router = build_router(&config.settings);
            let ctx = AiContext::default();
            match router.explain_command(&command, &ctx).await {
                Ok(text) => println!("{text}"),
                Err(e) => {
                    eprintln!("AI error: {e}");
                    println!("{}", wc_ai::router::stub_explain(&command));
                }
            }
        }
        Commands::Update => {
            let seed = seed_path();
            if seed.exists() {
                std::fs::copy(&seed, &path)?;
                println!("Updated database from {}", seed.display());
            } else {
                println!("Seed database not found at {}", seed.display());
            }
        }
        Commands::Settings { command } => {
            let command = command.unwrap_or(SettingsCmd::List {
                json: false,
                raw: false,
            });
            handle_settings(&command)?;
        }
        Commands::Config { command } => {
            handle_config(&command)?;
        }
        Commands::Model { command } => {
            handle_model(&command).await?;
        }
        Commands::Completions { shell } => {
            use clap_complete::generate;
            let mut cmd = Cli::command();
            generate(shell, &mut cmd, "wc", &mut std::io::stdout());
        }
    }
    Ok(())
}

fn handle_settings(cmd: &SettingsCmd) -> Result<(), Box<dyn std::error::Error>> {
    let cfg_path = wc_core::config::config_path()?;
    match cmd {
        SettingsCmd::List { json, raw } => {
            let config = load_config()?;
            println!("config: {}", cfg_path.display());
            if *json {
                let map = settings_json_map(&config.settings, *raw);
                println!("{}", serde_json::to_string_pretty(&map)?);
            } else {
                print_settings_summary(&config.settings);
            }
        }
        SettingsCmd::Show { key, raw } => {
            let config = load_config()?;
            let key = match resolve_key(key) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            let value = get_key(&config.settings, key);
            println!("{}", display_value(value.as_deref(), key, *raw));
        }
        SettingsCmd::Set { key, value } => {
            let key = match resolve_key(key) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(2);
                }
            };
            let mut config = load_config()?;
            let value = normalize_value(key, value);
            if value.is_empty() {
                clear_key(&mut config.settings, key)?;
            } else if let Err(e) = set_key(&mut config.settings, key, value) {
                eprintln!("{e}");
                std::process::exit(2);
            }
            save_config(&config)?;
            let current = get_key(&config.settings, key);
            let displayed = display_value(current.as_deref(), key, false);
            println!("{} = {}", key, displayed);
            println!("saved to {}", cfg_path.display());
        }
        SettingsCmd::Edit => {
            let editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| String::from(""));
            if editor.is_empty() {
                eprintln!(
                    "no $EDITOR or $VISUAL set; open manually: {}",
                    cfg_path.display()
                );
                std::process::exit(2);
            }
            println!("opening {} with {}", cfg_path.display(), editor);
            let mut child = std::process::Command::new(&editor)
                .arg(cfg_path.as_os_str())
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?;
            let status = child.wait()?;
            if !status.success() {
                eprintln!("editor exited with {status}");
                std::process::exit(status.code().unwrap_or(1));
            }
        }
        SettingsCmd::Reset => {
            let config = wc_core::models::AppSettings::default();
            let full = wc_core::config::AppConfig { settings: config };
            save_config(&full)?;
            println!(
                "reset config to defaults and saved to {}",
                cfg_path.display()
            );
        }
        SettingsCmd::Env => {
            let vars = [
                ("OPENCODE_API_KEY", "opencode_api_key"),
                ("KILO_API_KEY", "kilo_api_key"),
                ("LOCAL_GGUF_PATH", "local_model_path"),
                ("OPENAI_COMPAT_BASE_URL", "openai_compat_base_url"),
                ("OPENAI_COMPAT_API_KEY", "openai_compat_api_key"),
            ];
            let mut set = false;
            for (env, key) in vars {
                match std::env::var(env) {
                    Ok(v) => {
                        set = true;
                        if is_secret_key(key) {
                            println!("{env} = {}", mask(&v));
                        } else {
                            println!("{env} = {v}");
                        }
                    }
                    Err(std::env::VarError::NotPresent) => {}
                    Err(_) => {
                        set = true;
                        println!("{env} = <unset>");
                    }
                }
            }
            if !set {
                println!("no override env vars set (env vars take precedence over config.toml)");
            }
        }
    }
    Ok(())
}

fn handle_config(cmd: &ConfigCmd) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ConfigCmd::Path => {
            let path = wc_core::config::config_path()?;
            println!("{}", path.display());
        }
        ConfigCmd::Dir => {
            let dir = wc_core::config::config_dir()?;
            println!("{}", dir.display());
        }
    }
    Ok(())
}

/// Known on-device GGUF models and their download URLs.
/// Extend here when adding new bundled defaults. Env `WC_MODEL_BASE_URL`
/// overrides the host (useful behind mirrors).
const MODEL_URLS: &[(&str, &str, &str)] = &[
    ("gemma-2b-it-q4", "google/gemma-2b-it-q4_0-gguf", "gemma-2b-it-q4_0.gguf"),
    ("gemma-2b-it-q4_0", "google/gemma-2b-it-q4_0-gguf", "gemma-2b-it-q4_0.gguf"),
    ("gemma-2b-it-q8_0", "google/gemma-2b-it-q8_0-gguf", "gemma-2b-it-q8_0.gguf"),
];

fn model_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let dir = wc_core::config::config_dir()?.join("models");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn model_url(id: &str) -> Option<String> {
    if let Some(entry) = MODEL_URLS.iter().find(|(k, _, _)| *k == id) {
        let host = std::env::var("WC_MODEL_BASE_URL")
            .ok()
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "https://huggingface.co".to_string());
        return Some(format!(
            "{}/{}/resolve/main/{}",
            host.trim_end_matches('/'),
            entry.1,
            entry.2
        ));
    }
    // Allow a raw URL or a local path to be passed directly.
    if id.starts_with("http://") || id.starts_with("https://") || std::path::Path::new(id).is_file()
    {
        return Some(id.to_string());
    }
    None
}

fn human_bytes(n: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    if n == 0 {
        return "0 B".into();
    }
    let mut size = n as f64;
    let mut idx = 0;
    while size >= 1024.0 && idx < UNITS.len() - 1 {
        size /= 1024.0;
        idx += 1;
    }
    if idx == 0 {
        format!("{:.0} {}", size, UNITS[idx])
    } else {
        format!("{:.1} {}", size, UNITS[idx])
    }
}

async fn handle_model(cmd: &ModelCmd) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        ModelCmd::Download { id, skip_existing } => {
            let model_id = id.clone().unwrap_or_else(default_model_id);
            let url = match model_url(&model_id) {
                Some(u) => u,
                None => {
                    eprintln!(
                        "unknown model '{model_id}'. Known: {}",
                        MODEL_URLS.iter().map(|(k, _, _)| *k).collect::<Vec<_>>().join(", ")
                    );
                    eprintln!("or pass a full URL / local file path.");
                    std::process::exit(2);
                }
            };
            download_model(&model_id, &url, *skip_existing).await?;
        }
        ModelCmd::List => {
            let dir = model_dir()?;
            println!("models dir: {}", dir.display());
            println!("known models:");
            for (k, _, file) in MODEL_URLS {
                let local = dir.join(format!("{k}.gguf"));
                let size = if local.is_file() {
                    std::fs::metadata(&local)
                        .ok()
                        .map(|m| human_bytes(m.len()))
                        .unwrap_or_else(|| "<unknown>".into())
                } else {
                    "<not downloaded>".into()
                };
                println!("  {k}\t{size}\t({file})");
            }
            if dir.is_dir() {
                for entry in std::fs::read_dir(&dir)? {
                    let entry = entry?;
                    let name = entry.file_name();
                    let name = name.to_string_lossy();
                    if name.ends_with(".gguf")
                        && !MODEL_URLS.iter().any(|(k, _, _)| name == format!("{k}.gguf"))
                    {
                        let size = entry
                            .metadata()
                            .ok()
                            .map(|m| human_bytes(m.len()))
                            .unwrap_or_else(|| "?".into());
                        println!("  {name} *\t{size}");
                    }
                }
            }
        }
    }
    Ok(())
}

async fn download_model(
    model_id: &str,
    url: &str,
    skip_existing: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = model_dir()?;
    let dest = dir.join(format!("{model_id}.gguf"));

    let client = reqwest::Client::builder()
        .user_agent(format!("wc-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    let mut resp = client.get(url).send().await?;
    let status = resp.status();
    if !status.is_success() {
        eprintln!("HTTP {status} for {url}");
        std::process::exit(1);
    }
    let total = resp
        .content_length()
        .unwrap_or(0);
    if skip_existing && dest.exists() {
        if let Ok(meta) = dest.metadata() {
            if total == 0 || meta.len() == total {
                println!("{model_id}: already downloaded at {}", dest.display());
                return Ok(());
            }
        }
    }

    let tmp = dir.join(format!(".{model_id}.gguf.part"));
    let mut file = std::fs::File::create(&tmp)?;
    let mut downloaded: u64 = 0;
    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        if total > 0 {
            let pct = downloaded as f64 / total as f64 * 100.0;
            eprintln!(
                "\r  {model_id}: {}/{} ({:.0}%)    ",
                human_bytes(downloaded),
                human_bytes(total),
                pct
            );
        } else {
            eprintln!("\r  {model_id}: {} downloaded    ", human_bytes(downloaded));
        }
    }
    eprintln!();
    file.flush()?;
    std::fs::rename(&tmp, &dest)?;
    println!("{model_id}: downloaded {} to {}", human_bytes(downloaded), dest.display());
    println!(
        "enable with: wc settings set local_model_id {}\n   or:        wc settings set local_model_path {}",
        model_id, dest.display()
    );
    Ok(())
}

fn default_model_id() -> String {
    let config = load_config().ok();
    config
        .and_then(|c| c.settings.local_model_id.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "gemma-2b-it-q4".to_string())
}

fn normalize_value(key: &str, value: &str) -> String {
    // A bare "null" or empty string clears the key (enables fallback to env).
    if value.eq_ignore_ascii_case("null") || value.is_empty() {
        return String::new();
    }
    // ai_provider / fallback_provider accept lowercase aliases.
    if matches!(key, "ai_provider" | "fallback_provider") {
        return match value.to_ascii_lowercase().as_str() {
            "opencode" | "zen" | "opencode_zen" => "opencode_zen".into(),
            "kilo" | "gateway" | "kilo_gateway" => "kilo_gateway".into(),
            "local" | "gguf" | "local_llm" => "local_llm".into(),
            "openai" | "compat" | "openai_compat" => "openai_compat".into(),
            _ => value.to_string(),
        };
    }
    value.to_string()
}

fn settings_json_map(
    settings: &wc_core::models::AppSettings,
    raw: bool,
) -> serde_json::Map<String, serde_json::Value> {
    let mut map = serde_json::Map::new();
    for key in VALID_KEYS {
        let value = get_key(settings, key);
        let entry = match value {
            None => serde_json::Value::Null,
            Some(v) => {
                if is_secret_key(key) && !raw {
                    serde_json::Value::String(mask(&v))
                } else if *key == "local_max_tokens" {
                    v.parse::<u64>()
                        .map(serde_json::Value::from)
                        .unwrap_or_else(|_| serde_json::Value::String(v))
                } else {
                    serde_json::Value::String(v)
                }
            }
        };
        map.insert((*key).to_string(), entry);
    }
    map
}

fn print_settings_summary(settings: &wc_core::models::AppSettings) {
    let fallback_provider = settings.fallback_provider.as_deref().unwrap_or("<none>");
    let fallback_model = settings.fallback_model.as_deref().unwrap_or("<none>");
    let local_model_id = settings.local_model_id.as_deref().unwrap_or("<none>");
    let local_model_path = settings.local_model_path.as_deref().unwrap_or("<none>");
    let openai_compat_base_url = settings
        .openai_compat_base_url
        .as_deref()
        .unwrap_or("<none>");
    let local_max_tokens = settings
        .local_max_tokens
        .map_or("<none>".to_string(), |n| n.to_string());
    let opencode_api_key = mask_secret(&settings.opencode_api_key);
    let kilo_api_key = mask_secret(&settings.kilo_api_key);
    let openai_compat_api_key = mask_secret(&settings.openai_compat_api_key);
    let fields: [(&str, &str); 11] = [
        ("ai_provider", settings.ai_provider.as_str()),
        ("ai_model", settings.ai_model.as_str()),
        ("fallback_provider", fallback_provider),
        ("fallback_model", fallback_model),
        ("opencode_api_key", &opencode_api_key),
        ("kilo_api_key", &kilo_api_key),
        ("local_model_id", local_model_id),
        ("local_model_path", local_model_path),
        ("local_max_tokens", &local_max_tokens),
        ("openai_compat_base_url", openai_compat_base_url),
        ("openai_compat_api_key", &openai_compat_api_key),
    ];
    let width = fields.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    for (key, value) in fields {
        println!("{:>w$}  {}", key, value, w = width);
    }
}

fn mask_secret(opt: &Option<String>) -> String {
    match opt {
        Some(v) => mask(v),
        None => "<none>".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_roundtrip() {
        let mut s = AppSettings::default();
        assert!(VALID_KEYS.contains(&"ai_provider"));

        set_key(&mut s, "ai_provider", "local_llm".into()).unwrap();
        assert_eq!(get_key(&s, "ai_provider").unwrap(), "local_llm");

        set_key(&mut s, "ai_model", "gemma-3".into()).unwrap();
        assert_eq!(get_key(&s, "ai_model").unwrap(), "gemma-3");

        set_key(&mut s, "opencode_api_key", "sk-real-secret".into()).unwrap();
        assert_eq!(get_key(&s, "opencode_api_key").unwrap(), "sk-real-secret");
        // masking hides the secret
        assert_eq!(mask(&get_key(&s, "opencode_api_key").unwrap()), "••••:cret");
    }

    #[test]
    fn set_rejects_unknown_key() {
        let mut s = AppSettings::default();
        assert!(set_key(&mut s, "bogus", "x".into()).is_err());
        assert!(get_key(&s, "bogus").is_none());
    }

    #[test]
    fn set_rejects_invalid_provider() {
        let mut s = AppSettings::default();
        let res = set_key(&mut s, "ai_provider", "unknown_ai".into());
        assert!(matches!(res, Err(ref e) if e.contains("invalid ai_provider")));
        // valid provider accepted
        set_key(&mut s, "ai_provider", "kilo_gateway".into()).unwrap();
        assert_eq!(get_key(&s, "ai_provider").unwrap(), "kilo_gateway");
    }

    #[test]
    fn normalize_aliases_providers() {
        assert_eq!(normalize_value("ai_provider", "opencode"), "opencode_zen");
        assert_eq!(normalize_value("ai_provider", "kilo"), "kilo_gateway");
        assert_eq!(normalize_value("ai_provider", "local"), "local_llm");
        assert_eq!(normalize_value("ai_provider", "openai"), "openai_compat");
        // already-canonical values pass through
        assert_eq!(
            normalize_value("ai_provider", "openai_compat"),
            "openai_compat"
        );
    }

    #[test]
    fn normalize_null_clears() {
        assert_eq!(normalize_value("opencode_api_key", "null"), "");
        assert_eq!(normalize_value("opencode_api_key", ""), "");
    }

    #[test]
    fn local_max_tokens_parses() {
        let mut s = AppSettings::default();
        set_key(&mut s, "local_max_tokens", "512".into()).unwrap();
        assert_eq!(get_key(&s, "local_max_tokens").unwrap(), "512".to_string());
        assert!(set_key(&mut s, "local_max_tokens", "not-a-num".into()).is_err());
    }

    #[test]
    fn clear_key_removes_option() {
        let mut s = AppSettings::default();
        set_key(&mut s, "opencode_api_key", "x".into()).unwrap();
        assert!(s.opencode_api_key.is_some());
        clear_key(&mut s, "opencode_api_key").unwrap();
        assert!(s.opencode_api_key.is_none());
    }

    #[test]
    fn get_key_treats_unset_as_none() {
        let s = AppSettings::default();
        assert_eq!(get_key(&s, "opencode_api_key"), None);
        assert_eq!(get_key(&s, "local_model_path"), None);
        assert_eq!(get_key(&s, "ai_provider"), Some("opencode_zen".to_string()));
    }

    #[test]
    fn set_then_show_roundtrip() {
        let mut s = AppSettings::default();
        set_key(&mut s, "ai_model", "gemma-3b-it".into()).unwrap();
        let v = get_key(&s, "ai_model");
        assert!(v.is_some());
        assert_eq!(v.unwrap(), "gemma-3b-it");
    }

    #[test]
    fn resolve_key_distinguishes_known_vs_unknown() {
        assert_eq!(resolve_key("ai_provider"), Ok("ai_provider"));
        assert!(resolve_key("bogus").is_err());
    }

    #[test]
    fn display_value_masks_and_handles_none() {
        // secret masked unless raw
        assert_eq!(
            display_value(Some("sk-secret1234"), "opencode_api_key", false),
            "••••:1234"
        );
        assert_eq!(
            display_value(Some("sk-secret1234"), "opencode_api_key", true),
            "sk-secret1234"
        );
        // non-secret plain
        assert_eq!(
            display_value(Some("openai_compat"), "ai_provider", false),
            "openai_compat"
        );
        // None (valid-but-unset) -> "<none>", not "unknown key"
        assert_eq!(display_value(None, "opencode_api_key", false), "<none>");
        // empty string -> "<none>"
        assert_eq!(display_value(Some(""), "ai_model", false), "<none>");
    }

    #[test]
    fn json_map_masks_secrets_by_default() {
        let mut s = AppSettings::default();
        set_key(&mut s, "opencode_api_key", "sk-real-secret-1234".into()).unwrap();
        set_key(&mut s, "kilo_api_key", "kk-secret-5678".into()).unwrap();
        set_key(&mut s, "ai_model", "mimo-v2.5-free".into()).unwrap();
        set_key(&mut s, "local_max_tokens", "512".into()).unwrap();

        let masked = settings_json_map(&s, false);
        assert_eq!(
            masked.get("opencode_api_key"),
            Some(&serde_json::Value::String("••••:1234".into()))
        );
        assert_eq!(
            masked.get("kilo_api_key"),
            Some(&serde_json::Value::String("••••:5678".into()))
        );
        // non-secret values are plain
        assert_eq!(
            masked.get("ai_model"),
            Some(&serde_json::Value::String("mimo-v2.5-free".into()))
        );
        // local_max_tokens is a JSON number
        assert_eq!(
            masked.get("local_max_tokens"),
            Some(&serde_json::Value::Number(512.into()))
        );
        // unset optionals are null, not "<none>"
        assert_eq!(
            masked.get("local_model_path"),
            Some(&serde_json::Value::Null)
        );
    }

    #[test]
    fn json_map_raw_reveals_secrets() {
        let mut s = AppSettings::default();
        set_key(&mut s, "opencode_api_key", "sk-real-secret-1234".into()).unwrap();

        let raw = settings_json_map(&s, true);
        assert_eq!(
            raw.get("opencode_api_key"),
            Some(&serde_json::Value::String("sk-real-secret-1234".into()))
        );
        // non-secrets unchanged
        assert_eq!(
            raw.get("ai_provider"),
            Some(&serde_json::Value::String("opencode_zen".into()))
        );
    }

    #[test]
    fn json_map_includes_all_known_keys() {
        let s = AppSettings::default();
        let map = settings_json_map(&s, false);
        for key in VALID_KEYS {
            assert!(map.contains_key(*key), "missing key in JSON map: {key}");
        }
        assert_eq!(map.len(), VALID_KEYS.len());
    }

    #[test]
    fn mask_handles_short_and_long() {
        assert_eq!(mask("ab"), "••••");
        assert_eq!(mask("secret1234"), "••••:1234");
    }

    #[test]
    fn config_path_and_dir_resolve() {
        let dir = wc_core::config::config_dir().expect("config dir resolves");
        let path = wc_core::config::config_path().expect("config path resolves");
        // config.toml lives under the config directory
        assert_eq!(path.parent().unwrap(), dir.as_path());
        assert_eq!(path.file_name().unwrap().to_str().unwrap(), "config.toml");
        // the last path component is the "what-command" directory
        assert_eq!(dir.file_name().unwrap().to_str().unwrap(), "what-command");
    }

    #[test]
    fn model_url_known() {
        let u = model_url("gemma-2b-it-q4").expect("gemma-2b-it-q4 known");
        assert!(u.ends_with("google/gemma-2b-it-q4_0-gguf/resolve/main/gemma-2b-it-q4_0.gguf"));
    }

    #[test]
    fn model_url_respects_base_env() {
        std::env::set_var("WC_MODEL_BASE_URL", "https://mirror.example/hf");
        let u = model_url("gemma-2b-it-q4").expect("url from mirror");
        assert!(u.starts_with("https://mirror.example/hf/"));
        assert!(u.ends_with("google/gemma-2b-it-q4_0-gguf/resolve/main/gemma-2b-it-q4_0.gguf"));
        std::env::remove_var("WC_MODEL_BASE_URL");
    }

    #[test]
    fn model_url_accepts_raw_url_and_local_path() {
        assert!(model_url("https://example.com/x.gguf").is_some());
        // local file path
        let tmp = std::env::temp_dir().join("wc-gguf-dummy.gguf");
        std::fs::write(&tmp, b"").unwrap();
        assert!(model_url(tmp.to_string_lossy().as_ref()).is_some());
        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn model_url_unknown_is_none() {
        std::env::remove_var("WC_MODEL_BASE_URL");
        assert!(model_url("no-such-model").is_none());
    }

    #[test]
    fn default_model_id_is_known_when_unset() {
        std::env::remove_var("WC_MODEL_BASE_URL");
        // AppSettings defaults may be empty in a fresh env; ensure a sane default.
        let id = default_model_id();
        assert!(!id.is_empty());
    }
}
