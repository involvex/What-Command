use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::error::{Result, WcError};
use crate::models::AppSettings;

static CONFIG_DIR: OnceLock<PathBuf> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub settings: AppSettings,
}

/// Override config directory (required on Android where `dirs::config_dir` is unreliable).
pub fn set_config_dir(path: PathBuf) {
    let _ = CONFIG_DIR.set(path);
}

pub fn config_dir() -> Result<PathBuf> {
    if let Some(dir) = CONFIG_DIR.get() {
        return Ok(dir.clone());
    }
    dirs::config_dir()
        .map(|p| p.join("what-command"))
        .ok_or_else(|| WcError::Config("could not resolve config directory".into()))
}

pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

pub fn load_config() -> Result<AppConfig> {
    let path = config_path()?;
    let mut config = if !path.exists() {
        AppConfig::default()
    } else {
        let raw = fs::read_to_string(&path)?;
        toml::from_str(&raw).map_err(|e| WcError::Config(e.to_string()))?
    };
    merge_env_keys(&mut config.settings);
    Ok(config)
}

fn merge_env_keys(settings: &mut AppSettings) {
    if settings.opencode_api_key.as_ref().is_none_or(|k| k.is_empty()) {
        if let Ok(v) = std::env::var("OPENCODE_API_KEY") {
            if !v.is_empty() {
                settings.opencode_api_key = Some(v);
            }
        }
    }
    if settings.kilo_api_key.as_ref().is_none_or(|k| k.is_empty()) {
        if let Ok(v) = std::env::var("KILO_API_KEY") {
            if !v.is_empty() {
                settings.kilo_api_key = Some(v);
            }
        }
    }
    if settings.local_model_path.as_ref().is_none_or(|p| p.is_empty()) {
        if let Ok(v) = std::env::var("LOCAL_GGUF_PATH") {
            if !v.is_empty() {
                settings.local_model_path = Some(v);
            }
        }
    }
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let dir = config_dir()?;
    fs::create_dir_all(&dir)?;
    let raw = toml::to_string_pretty(config).map_err(|e| WcError::Config(e.to_string()))?;
    fs::write(config_path()?, raw)?;
    Ok(())
}

pub fn ensure_db_from_seed(seed: &Path, target: &Path) -> Result<()> {
    ensure_db_from_seed_source(
        seed.exists().then_some(SeedSource::Path(seed)),
        target,
    )
}

pub fn ensure_db_from_seed_bytes(seed: &[u8], target: &Path) -> Result<()> {
    ensure_db_from_seed_source(Some(SeedSource::Bytes(seed)), target)
}

enum SeedSource<'a> {
    Path(&'a Path),
    Bytes(&'a [u8]),
}

fn ensure_db_from_seed_source(source: Option<SeedSource<'_>>, target: &Path) -> Result<()> {
    if target.exists() {
        match db_quick_check(target) {
            Ok(true) => return Ok(()),
            Ok(false) | Err(_) => {
                fs::remove_file(target)?;
            }
        }
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    if let Some(source) = source {
        let copied = match source {
            SeedSource::Path(seed) => fs::copy(seed, target).is_ok(),
            SeedSource::Bytes(bytes) => fs::write(target, bytes).is_ok(),
        };
        if copied && db_quick_check(target).unwrap_or(false) {
            return Ok(());
        }
        let _ = fs::remove_file(target);
    }
    create_demo_db(target)
}

fn create_demo_db(target: &Path) -> Result<()> {
    let store = crate::db::CommandStore::open(target)?;
    store.init_schema()?;
    store.seed_demo()?;
    Ok(())
}

fn db_quick_check(path: &Path) -> Result<bool> {
    let conn = rusqlite::Connection::open(path)?;
    let status: String = conn.query_row("PRAGMA quick_check", [], |row| row.get(0))?;
    Ok(status == "ok")
}
