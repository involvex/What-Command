use async_trait::async_trait;
use tokio::task;

use crate::local_engine::{complete, resolve_model_path};
use crate::providers::opencode_zen::parse_suggestion;
use crate::AiProvider;
use wc_core::error::{Result, WcError};
use wc_core::models::{AiContext, CommandSuggestion};

const SYSTEM: &str =
    "You are What Command, a CLI assistant. Respond with a single shell command on the first line, then a one-line explanation.";

/// On-device GGUF inference via llama.cpp (feature `local-llm`).
pub struct LocalLlmProvider {
    model_id: String,
    model_path: Option<String>,
    max_tokens: u32,
}

impl LocalLlmProvider {
    pub fn new(model_id: String, model_path: Option<String>, max_tokens: Option<u32>) -> Self {
        Self {
            model_id,
            model_path,
            max_tokens: max_tokens.unwrap_or(256),
        }
    }

    fn resolved_path(&self) -> Option<std::path::PathBuf> {
        resolve_model_path(&self.model_id, self.model_path.as_deref())
    }

    async fn run_prompt(&self, user: &str) -> Result<String> {
        let path = self
            .resolved_path()
            .ok_or_else(|| {
                WcError::Ai(format!(
                    "no GGUF model found for '{}'. Set local_model_path or place {}.gguf in ~/.config/what-command/models/",
                    self.model_id, self.model_id
                ))
            })?;
        let prompt = format!("{SYSTEM}\n\n{user}");
        let max_tokens = self.max_tokens;
        task::spawn_blocking(move || complete(&path, &prompt, max_tokens))
            .await
            .map_err(|e| WcError::Ai(e.to_string()))?
    }
}

#[async_trait]
impl AiProvider for LocalLlmProvider {
    fn id(&self) -> &'static str {
        "local_llm"
    }

    async fn generate_command(&self, prompt: &str, ctx: &AiContext) -> Result<CommandSuggestion> {
        let fw = ctx
            .framework_id
            .as_deref()
            .unwrap_or("shell");
        let user = format!("Generate a shell command for ({fw}): {prompt}");
        match self.run_prompt(&user).await {
            Ok(text) => Ok(parse_suggestion(&text)),
            Err(e) => Ok(CommandSuggestion {
                command: format!("echo 'local LLM unavailable: {e}'"),
                explanation: e.to_string(),
                danger_level: 0,
                platforms: vec!["common".into()],
            }),
        }
    }

    async fn explain_command(&self, cmd: &str, _ctx: &AiContext) -> Result<String> {
        let user = format!("Explain this shell command briefly for beginners: {cmd}");
        self.run_prompt(&user).await
    }
}

pub fn unavailable() -> wc_core::error::WcError {
    WcError::Ai("local LLM not configured".into())
}
