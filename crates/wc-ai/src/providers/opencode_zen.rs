use async_trait::async_trait;

use crate::openai_compat::OpenAiCompatClient;
use crate::AiProvider;
use wc_core::error::Result;
use wc_core::models::{AiContext, CommandSuggestion};

const SYSTEM: &str = "You are What Command, a CLI assistant. Respond with a single shell command and a one-line explanation. Format: first line command only, then explanation.";

pub struct OpenCodeZenProvider {
    inner: OpenAiCompatClient,
}

impl OpenCodeZenProvider {
    pub fn new(api_key: Option<String>, model: String) -> Self {
        Self {
            inner: OpenAiCompatClient::new("https://opencode.ai/zen/v1", api_key, model),
        }
    }
}

#[async_trait]
impl AiProvider for OpenCodeZenProvider {
    fn id(&self) -> &'static str {
        "opencode_zen"
    }

    async fn generate_command(&self, prompt: &str, _ctx: &AiContext) -> Result<CommandSuggestion> {
        let text = self
            .inner
            .chat(SYSTEM, &format!("Generate a shell command for: {prompt}"))
            .await?;
        Ok(parse_suggestion(&text))
    }

    async fn explain_command(&self, cmd: &str, _ctx: &AiContext) -> Result<String> {
        self.inner
            .chat(
                "Explain shell commands briefly for beginners.",
                &format!("Explain: {cmd}"),
            )
            .await
    }
}

pub fn parse_suggestion(text: &str) -> CommandSuggestion {
    let mut lines = text.lines();
    let command = lines.next().unwrap_or("echo hello").trim().to_string();
    let explanation = lines.collect::<Vec<_>>().join(" ").trim().to_string();
    CommandSuggestion {
        command,
        explanation: if explanation.is_empty() {
            "AI-generated command".into()
        } else {
            explanation
        },
        danger_level: 0,
        platforms: vec!["common".into()],
    }
}
