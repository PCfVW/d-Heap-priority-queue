//! Mistral AI Provider

use crate::provider::{LlmProvider, LlmResponse, RequestConfig};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const MISTRAL_API_URL: &str = "https://api.mistral.ai/v1/chat/completions";

pub struct MistralProvider {
    api_key: String,
    client: reqwest::Client,
}

impl MistralProvider {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("MISTRAL_API_KEY")
            .map_err(|_| anyhow!("MISTRAL_API_KEY environment variable not set"))?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()?;

        Ok(Self { api_key, client })
    }
}

// OpenAI-compatible request/response format
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
    model: String,
    usage: ChatUsage,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatUsage {
    prompt_tokens: usize,
    completion_tokens: usize,
}

#[async_trait]
impl LlmProvider for MistralProvider {
    fn default_model(&self) -> &str {
        "mistral-medium-latest"
    }

    async fn complete(&self, prompt: &str, config: &RequestConfig) -> Result<LlmResponse> {
        let model = config
            .model
            .as_deref()
            .unwrap_or_else(|| self.default_model());

        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: config.max_tokens,
            temperature: config.temperature,
        };

        let response = self
            .client
            .post(MISTRAL_API_URL)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("Mistral API error ({}): {}", status, error_text));
        }

        let result: ChatResponse = response.json().await?;

        let content = result
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(LlmResponse {
            content,
            input_tokens: result.usage.prompt_tokens,
            output_tokens: result.usage.completion_tokens,
            model: result.model,
            provider: "mistral".to_string(),
        })
    }
}
