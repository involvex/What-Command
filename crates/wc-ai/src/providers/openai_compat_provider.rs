use async_trait::async_trait;

use crate::openai_compat::OpenAiCompatClient;
use crate::providers::opencode_zen::parse_suggestion;
use crate::AiProvider;
use wc_core::error::Result;
use wc_core::models::{AiContext, CommandSuggestion};

pub struct OpenAiCompatProvider {
    inner: OpenAiCompatClient,
}

impl OpenAiCompatProvider {
    pub fn new(base_url: String, api_key: Option<String>, model: String) -> Self {
        Self {
            inner: OpenAiCompatClient::new(base_url, api_key, model),
        }
    }
}

#[async_trait]
impl AiProvider for OpenAiCompatProvider {
    fn id(&self) -> &'static str {
        "openai_compat"
    }

    async fn generate_command(&self, prompt: &str, _ctx: &AiContext) -> Result<CommandSuggestion> {
        let text = self
            .inner
            .chat(
                "Return one shell command and explanation.",
                prompt,
            )
            .await?;
        Ok(parse_suggestion(&text))
    }

    async fn explain_command(&self, cmd: &str, _ctx: &AiContext) -> Result<String> {
        self.inner
            .chat("Explain shell commands.", cmd)
            .await
    }
}
