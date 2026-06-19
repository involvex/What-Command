use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub command: String,
    pub description: String,
    pub category: String,
    pub platform: Vec<String>,
    pub danger_level: u8,
    pub source: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Framework {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandSuggestion {
    pub command: String,
    pub explanation: String,
    pub danger_level: u8,
    pub platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateResult {
    pub explanation: String,
    pub sample_output: String,
    pub exit_code: i32,
    pub blocked: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiContext {
    pub framework_id: Option<String>,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaygroundSession {
    pub id: String,
    pub command: String,
    pub transcript: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub ai_provider: String,
    pub ai_model: String,
    pub fallback_provider: Option<String>,
    pub fallback_model: Option<String>,
    pub opencode_api_key: Option<String>,
    pub kilo_api_key: Option<String>,
    pub local_model_id: Option<String>,
    pub local_model_path: Option<String>,
    pub local_max_tokens: Option<u32>,
    pub openai_compat_base_url: Option<String>,
    pub openai_compat_api_key: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            ai_provider: "opencode_zen".into(),
            ai_model: "gpt-4o-mini".into(),
            fallback_provider: Some("local_llm".into()),
            fallback_model: Some("gemma-2b".into()),
            opencode_api_key: None,
            kilo_api_key: None,
            local_model_id: Some("gemma-2b-it-q4".into()),
            local_model_path: None,
            local_max_tokens: Some(256),
            openai_compat_base_url: None,
            openai_compat_api_key: None,
        }
    }
}
