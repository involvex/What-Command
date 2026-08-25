use async_trait::async_trait;

use crate::openai_compat::OpenAiCompatClient;
use crate::AiProvider;
use wc_core::error::Result;
use wc_core::models::{AiContext, CommandSuggestion};

use super::opencode_zen::parse_suggestion;

const SYSTEM: &str = "You are What Command. Return one shell command and short explanation.";

pub struct KiloGatewayProvider {
    inner: OpenAiCompatClient,
}

impl KiloGatewayProvider {
    pub fn new(api_key: Option<String>, model: String) -> Self {
        Self {
            inner: OpenAiCompatClient::new(
                "https://api.kilo.ai/api/gateway",
                api_key,
                model,
            )
            .with_header("x-kilocode-mode", "balanced"),
        }
    }
}

#[async_trait]
impl AiProvider for KiloGatewayProvider {
    fn id(&self) -> &'static str {
        "kilo_gateway"
    }

    async fn generate_command(&self, prompt: &str, ctx: &AiContext) -> Result<CommandSuggestion> {
        let fw = ctx.framework_id.as_deref().unwrap_or("shell");
        let text = self
            .inner
            .chat(SYSTEM, &format!("Generate a shell command for ({fw}): {prompt}"))
            .await?;
        Ok(parse_suggestion(&text))
    }

    async fn explain_command(&self, cmd: &str, _ctx: &AiContext) -> Result<String> {
        self.inner
            .chat("Explain CLI commands concisely.", &format!("Command: {cmd}"))
            .await
    }
}
