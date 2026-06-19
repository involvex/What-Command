use serde::{Deserialize, Serialize};

use wc_core::error::{Result, WcError};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

pub struct OpenAiCompatClient {
    client: reqwest::Client,
    base_url: String,
    api_key: Option<String>,
    model: String,
    extra_headers: Vec<(String, String)>,
}

impl OpenAiCompatClient {
    pub fn new(base_url: impl Into<String>, api_key: Option<String>, model: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            api_key,
            model: model.into(),
            extra_headers: vec![],
        }
    }

    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra_headers.push((key.into(), value.into()));
        self
    }

    pub async fn chat(&self, system: &str, user: &str) -> Result<String> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = ChatRequest {
            model: self.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: system.into(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: user.into(),
                },
            ],
            temperature: 0.2,
        };

        let mut req = self.client.post(&url).json(&body);
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }
        for (k, v) in &self.extra_headers {
            req = req.header(k, v);
        }

        let resp = req
            .send()
            .await
            .map_err(|e| WcError::Ai(e.to_string()))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(WcError::Ai(format!("HTTP {status}: {text}")));
        }

        let parsed: ChatResponse = resp
            .json()
            .await
            .map_err(|e| WcError::Ai(e.to_string()))?;
        parsed
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| WcError::Ai("empty response".into()))
    }
}
