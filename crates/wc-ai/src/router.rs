use std::sync::Arc;

use crate::providers::{
    kilo_gateway::KiloGatewayProvider, local_llm::LocalLlmProvider,
    opencode_zen::OpenCodeZenProvider, openai_compat_provider::OpenAiCompatProvider,
};
use crate::AiProvider;
use wc_core::error::Result;
use wc_core::models::{AiContext, AppSettings, CommandSuggestion};

pub struct AiRouter {
    primary: Arc<dyn AiProvider>,
    fallback: Option<Arc<dyn AiProvider>>,
}

#[derive(Clone)]
struct ProviderConfig {
    opencode_key: Option<String>,
    kilo_key: Option<String>,
    model: String,
    local_model: Option<String>,
    local_path: Option<String>,
    local_max_tokens: Option<u32>,
    compat_base: Option<String>,
    compat_key: Option<String>,
}

impl AiRouter {
    pub fn from_settings(settings: &AppSettings) -> Self {
        let shared = ProviderConfig {
            opencode_key: settings.opencode_api_key.clone(),
            kilo_key: settings.kilo_api_key.clone(),
            local_model: settings.local_model_id.clone(),
            local_path: settings.local_model_path.clone(),
            local_max_tokens: settings.local_max_tokens,
            compat_base: settings.openai_compat_base_url.clone(),
            compat_key: settings.openai_compat_api_key.clone(),
            model: String::new(),
        };
        let primary = provider_from_name(
            &settings.ai_provider,
            ProviderConfig {
                model: settings.ai_model.clone(),
                ..shared.clone()
            },
        );
        let fallback = settings.fallback_provider.as_ref().map(|name| {
            provider_from_name(
                name,
                ProviderConfig {
                    model: settings
                        .fallback_model
                        .clone()
                        .unwrap_or_else(|| "gemma-2b".into()),
                    ..shared
                },
            )
        });
        Self { primary, fallback }
    }

    pub async fn generate_command(&self, prompt: &str, ctx: &AiContext) -> Result<CommandSuggestion> {
        match self.primary.generate_command(prompt, ctx).await {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Some(fb) = &self.fallback {
                    fb.generate_command(prompt, ctx).await.map_err(|_| e)
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn explain_command(&self, cmd: &str, ctx: &AiContext) -> Result<String> {
        match self.primary.explain_command(cmd, ctx).await {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Some(fb) = &self.fallback {
                    fb.explain_command(cmd, ctx).await.map_err(|_| e)
                } else {
                    Err(e)
                }
            }
        }
    }
}

fn provider_from_name(name: &str, cfg: ProviderConfig) -> Arc<dyn AiProvider> {
    match name {
        "kilo_gateway" => Arc::new(KiloGatewayProvider::new(cfg.kilo_key, cfg.model)),
        "local_llm" => Arc::new(LocalLlmProvider::new(
            cfg.local_model.unwrap_or_else(|| "gemma-2b".into()),
            cfg.local_path,
            cfg.local_max_tokens,
        )),
        "openai_compat" => Arc::new(OpenAiCompatProvider::new(
            cfg.compat_base
                .unwrap_or_else(|| "http://127.0.0.1:8080/v1".into()),
            cfg.compat_key,
            cfg.model,
        )),
        _ => Arc::new(OpenCodeZenProvider::new(cfg.opencode_key, cfg.model)),
    }
}

pub fn stub_suggestion(prompt: &str) -> CommandSuggestion {
    CommandSuggestion {
        command: format!("echo 'Configure AI provider for: {prompt}'"),
        explanation: "No API key configured — using offline placeholder.".into(),
        danger_level: 0,
        platforms: vec!["common".into()],
    }
}

pub fn stub_explain(cmd: &str) -> String {
    format!("Placeholder explanation for `{cmd}`. Set OPENCODE_API_KEY or KILO_API_KEY.")
}
