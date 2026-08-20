use std::path::PathBuf;
use clap::{Parser, Subcommand, CommandFactory};
use clap_complete::Shell;
use wc_core::config::{ensure_db_from_seed, load_config, save_config};
use wc_core::models::{AiContext, AppSettings};
use wc_core::db::CommandStore;
use wc_ai::build_router;

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
    Ask {
        prompt: String,
    },
    /// Explain a command via AI
    Explain {
        command: String,
    },
    /// Refresh bundled command database from seed
    Update,
    /// Configure AI providers, API keys, and models
    Settings {
        #[command(subcommand)]
        command: SettingsCmd,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
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
    Set {
        key: String,
        value: String,
    },
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../data/commands.db")
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
        "local_max_tokens" => settings.local_max_tokens = Some(value.parse().map_err(|e| {
            format!("local_max_tokens must be a number: {e}")
        })?),
        "openai_compat_base_url" => settings.openai_compat_base_url = Some(value),
        "openai_compat_api_key" => settings.openai_compat_api_key = Some(value),
        _ => return Err(format!("unknown key '{key}'. Valid: {}", VALID_KEYS.join(", "))),
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
            handle_settings(&command)?;
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
            if !VALID_KEYS.contains(&key.as_str()) {
                eprintln!("unknown key '{}'", key);
                eprintln!("valid keys: {}", VALID_KEYS.join(", "));
                std::process::exit(2);
            }
            let value = get_key(&config.settings, key);
            match value {
                Some(v) if !v.is_empty() => {
                    if is_secret_key(key) && !*raw {
                        println!("{}", mask(&v));
                    } else {
                        println!("{v}");
                    }
                }
                _ => println!("<none>"),
            }
        }
        SettingsCmd::Set { key, value } => {
            if !VALID_KEYS.contains(&key.as_str()) {
                eprintln!("unknown key '{}'", key);
                eprintln!("valid keys: {}", VALID_KEYS.join(", "));
                std::process::exit(2);
            }
            let mut config = load_config()?;
            let value = normalize_value(key, value);
            if value.is_empty() {
                clear_key(&mut config.settings, key)?;
            } else if let Err(e) = set_key(&mut config.settings, key, value) {
                eprintln!("{e}");
                std::process::exit(2);
            }
            save_config(&config)?;
            let current = get_key(&config.settings, key).unwrap_or_default();
            let displayed = if is_secret_key(key) {
                if current.is_empty() {
                    "<none>".to_string()
                } else {
                    mask(&current)
                }
            } else if current.is_empty() {
                "<none>".to_string()
            } else {
                current
            };
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
            let full = wc_core::config::AppConfig {
                settings: config,
            };
            save_config(&full)?;
            println!("reset config to defaults and saved to {}", cfg_path.display());
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
        assert_eq!(
            get_key(&s, "opencode_api_key").unwrap(),
            "sk-real-secret"
        );
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
        assert_eq!(
            normalize_value("ai_provider", "openai"),
            "openai_compat"
        );
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
        assert_eq!(
            get_key(&s, "local_max_tokens").unwrap(),
            "512".to_string()
        );
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
    fn json_map_masks_secrets_by_default() {
        let mut s = AppSettings::default();
        set_key(&mut s, "opencode_api_key", "sk-real-secret-1234".into()).unwrap();
        set_key(&mut s, "kilo_api_key", "kk-secret-5678".into()).unwrap();
        set_key(&mut s, "ai_model", "mimo-v2.5-free".into()).unwrap();
        set_key(&mut s, "local_max_tokens", "512".into()).unwrap();

        let masked = settings_json_map(&s, false);
        assert_eq!(masked.get("opencode_api_key"), Some(&serde_json::Value::String("••••:1234".into())));
        assert_eq!(masked.get("kilo_api_key"), Some(&serde_json::Value::String("••••:5678".into())));
        // non-secret values are plain
        assert_eq!(masked.get("ai_model"), Some(&serde_json::Value::String("mimo-v2.5-free".into())));
        // local_max_tokens is a JSON number
        assert_eq!(masked.get("local_max_tokens"), Some(&serde_json::Value::Number(512.into())));
        // unset optionals are null, not "<none>"
        assert_eq!(masked.get("local_model_path"), Some(&serde_json::Value::Null));
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
        assert_eq!(raw.get("ai_provider"), Some(&serde_json::Value::String("opencode_zen".into())));
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
}
