pub mod local_engine;
pub mod openai_compat;
pub mod providers;
pub mod router;

pub use openai_compat::OpenAiCompatClient;
pub use providers::{
    kilo_gateway::KiloGatewayProvider, local_llm::LocalLlmProvider,
    opencode_zen::OpenCodeZenProvider, openai_compat_provider::OpenAiCompatProvider,
};
pub use router::AiRouter;

use async_trait::async_trait;

use wc_core::error::Result;
use wc_core::models::{AiContext, AppSettings, CommandSuggestion};

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn id(&self) -> &'static str;
    async fn generate_command(&self, prompt: &str, ctx: &AiContext) -> Result<CommandSuggestion>;
    async fn explain_command(&self, cmd: &str, ctx: &AiContext) -> Result<String>;
}

pub fn build_router(settings: &AppSettings) -> AiRouter {
    AiRouter::from_settings(settings)
}
