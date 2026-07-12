use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
#[cfg(not(target_os = "android"))]
use std::path::Path;
use std::sync::Mutex;

use tauri::{Manager, State};
use wc_ai::build_router;
use wc_core::config::{
    config_dir, ensure_db_from_seed, load_config, save_config, set_config_dir, AppConfig,
};
#[cfg(target_os = "android")]
use wc_core::config::ensure_db_from_seed_bytes;
use wc_core::db::{simulate_with_store, CommandStore};
use wc_core::models::{
    AiContext, AppSettings, Command, Framework, PlaygroundSession, SimulateResult,
};

struct AppState {
    store: Mutex<CommandStore>,
    settings: Mutex<AppSettings>,
}

fn db_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("commands.db"))
}

fn seed_path(app: &tauri::AppHandle) -> PathBuf {
    if let Ok(resource_dir) = app.path().resource_dir() {
        let bundled = resource_dir.join("commands.db");
        if bundled.exists() {
            return bundled;
        }
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../data/commands.db")
}

#[cfg(target_os = "android")]
fn read_android_seed(app: &tauri::AppHandle) -> Option<Vec<u8>> {
    use tauri::path::BaseDirectory;
    use tauri_plugin_fs::FsExt;

    let resource_path = app
        .path()
        .resolve("commands.db", BaseDirectory::Resource)
        .ok()?;
    app.fs().read(&resource_path).ok()
}

fn init_state(app: &tauri::AppHandle) -> Result<AppState, String> {
    let config_dir = app.path().app_config_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    set_config_dir(config_dir);

    let path = db_path(app)?;

    #[cfg(target_os = "android")]
    {
        if let Some(seed_bytes) = read_android_seed(app) {
            ensure_db_from_seed_bytes(&seed_bytes, &path).map_err(|e| e.to_string())?;
        } else {
            ensure_db_from_seed(&seed_path(app), &path).map_err(|e| e.to_string())?;
        }
    }

    #[cfg(not(target_os = "android"))]
    {
        ensure_db_from_seed(&seed_path(app), &path).map_err(|e| e.to_string())?;
    }

    let store = CommandStore::open(&path).map_err(|e| e.to_string())?;
    store.init_schema().map_err(|e| e.to_string())?;
    let config = load_config().map_err(|e| e.to_string())?;
    Ok(AppState {
        store: Mutex::new(store),
        settings: Mutex::new(config.settings),
    })
}

#[tauri::command]
fn search_commands(state: State<'_, AppState>, query: String, limit: usize) -> Result<Vec<Command>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .search(&query, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_command(state: State<'_, AppState>, id: String) -> Result<Option<Command>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .get_by_id(&id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_categories(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .categories()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_frameworks(state: State<'_, AppState>) -> Result<Vec<Framework>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .list_frameworks()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn commands_by_framework(
    state: State<'_, AppState>,
    framework_id: String,
    limit: usize,
) -> Result<Vec<Command>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .commands_by_framework(&framework_id, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn related_commands(
    state: State<'_, AppState>,
    command_id: String,
    limit: usize,
) -> Result<Vec<Command>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .related_commands(&command_id, limit)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn validate_command(state: State<'_, AppState>, command: String) -> Result<(u8, String), String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .validate_command(&command)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn simulate_command_ipc(
    state: State<'_, AppState>,
    command: String,
    vars: HashMap<String, String>,
) -> Result<SimulateResult, String> {
    let store = state.store.lock().map_err(|e| e.to_string())?;
    Ok(simulate_with_store(&store, &command, &vars))
}

#[tauri::command]
async fn ask_ai(
    state: State<'_, AppState>,
    prompt: String,
    framework_id: Option<String>,
) -> Result<wc_core::models::CommandSuggestion, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?.clone();
    let router = build_router(&settings);
    let ctx = AiContext {
        framework_id,
        platform: None,
    };
    match router.generate_command(&prompt, &ctx).await {
        Ok(s) => Ok(s),
        Err(_) => Ok(wc_ai::router::stub_suggestion(&prompt)),
    }
}

#[tauri::command]
async fn explain_command(
    state: State<'_, AppState>,
    command: String,
) -> Result<String, String> {
    let settings = state.settings.lock().map_err(|e| e.to_string())?.clone();
    let router = build_router(&settings);
    let ctx = AiContext::default();
    match router.explain_command(&command, &ctx).await {
        Ok(s) => Ok(s),
        Err(_) => Ok(wc_ai::router::stub_explain(&command)),
    }
}

#[tauri::command]
fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    Ok(state.settings.lock().map_err(|e| e.to_string())?.clone())
}

#[tauri::command]
fn save_settings(state: State<'_, AppState>, mut settings: AppSettings) -> Result<(), String> {
    {
        let guard = state.settings.lock().map_err(|e| e.to_string())?;
        preserve_secrets(&guard, &mut settings);
    }
    {
        let mut guard = state.settings.lock().map_err(|e| e.to_string())?;
        *guard = settings.clone();
    }
    save_config(&AppConfig { settings }).map_err(|e| e.to_string())
}

fn preserve_secrets(previous: &AppSettings, next: &mut AppSettings) {
    if next.opencode_api_key.as_ref().is_none_or(|k| k.is_empty()) {
        next.opencode_api_key = previous.opencode_api_key.clone();
    }
    if next.kilo_api_key.as_ref().is_none_or(|k| k.is_empty()) {
        next.kilo_api_key = previous.kilo_api_key.clone();
    }
    if next.openai_compat_api_key.as_ref().is_none_or(|k| k.is_empty()) {
        next.openai_compat_api_key = previous.openai_compat_api_key.clone();
    }
}

#[cfg(target_os = "android")]
fn android_import_file_name(source_path: &str) -> Result<std::ffi::OsString, String> {
    use std::str::FromStr;

    use tauri_plugin_fs::FilePath;

    match FilePath::from_str(source_path).unwrap() {
        FilePath::Path(path) => path
            .file_name()
            .map(std::ffi::OsString::from)
            .ok_or_else(|| "invalid source path".to_string()),
        FilePath::Url(url) => {
            let segment = url
                .path()
                .rsplit('/')
                .find(|part| !part.is_empty())
                .ok_or_else(|| "invalid source URI".to_string())?;
            if segment.ends_with(".gguf") {
                return Ok(std::ffi::OsString::from(segment));
            }
            // SAF document ids often look like primary%3ADownload%2Fmodel.gguf
            let decoded = segment.replace("%2F", "/").replace("%3A", ":");
            let name = decoded
                .rsplit('/')
                .next()
                .filter(|name| name.ends_with(".gguf"))
                .or_else(|| decoded.rsplit(':').next().filter(|name| name.ends_with(".gguf")))
                .ok_or_else(|| {
                    "could not determine .gguf file name from picker URI; try another location"
                        .to_string()
                })?;
            Ok(std::ffi::OsString::from(name))
        }
    }
}

#[tauri::command]
#[cfg_attr(not(target_os = "android"), allow(unused_variables))]
async fn import_gguf_model(app: tauri::AppHandle, source_path: String) -> Result<String, String> {
    let dest_dir = config_dir().map_err(|e| e.to_string())?.join("models");
    fs::create_dir_all(&dest_dir).map_err(|e| e.to_string())?;

    #[cfg(target_os = "android")]
    let file_name = android_import_file_name(&source_path)?;
    #[cfg(not(target_os = "android"))]
    let file_name = Path::new(&source_path)
        .file_name()
        .map(|name| name.to_owned())
        .ok_or_else(|| "invalid source path".to_string())?;
    let dest = dest_dir.join(&file_name);

    #[cfg(target_os = "android")]
    {
        use std::str::FromStr;

        use tauri_plugin_fs::{FilePath, FsExt};

        let file_path = FilePath::from_str(&source_path).unwrap();
        let bytes = app
            .fs()
            .read(file_path)
            .map_err(|e| format!("failed to read model file: {e}"))?;
        fs::write(&dest, bytes).map_err(|e| e.to_string())?;
    }

    #[cfg(not(target_os = "android"))]
    {
        let source = PathBuf::from(&source_path);
        if !source.is_file() {
            return Err(format!("not a file: {source_path}"));
        }
        fs::copy(&source, &dest).map_err(|e| e.to_string())?;
    }

    Ok(dest.to_string_lossy().into_owned())
}

#[tauri::command]
fn save_playground_session(
    state: State<'_, AppState>,
    session: PlaygroundSession,
) -> Result<(), String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .save_playground_session(&session)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn list_playground_sessions(
    state: State<'_, AppState>,
    limit: usize,
) -> Result<Vec<PlaygroundSession>, String> {
    state
        .store
        .lock()
        .map_err(|e| e.to_string())?
        .list_playground_sessions(limit)
        .map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let state = init_state(app.handle())?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search_commands,
            get_command,
            list_categories,
            list_frameworks,
            commands_by_framework,
            related_commands,
            validate_command,
            simulate_command_ipc,
            ask_ai,
            explain_command,
            get_settings,
            save_settings,
            import_gguf_model,
            save_playground_session,
            list_playground_sessions,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
