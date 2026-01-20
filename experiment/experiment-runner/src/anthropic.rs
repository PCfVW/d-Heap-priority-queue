//! Anthropic Claude Provider

use crate::provider::{LlmProvider, LlmResponse, RequestConfig};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const ANTHROPIC_API_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_VERSION: &str = "2023-06-01";

pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
}

impl AnthropicProvider {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| anyhow!("ANTHROPIC_API_KEY environment variable not set"))?;

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 min for long generations
            .build()?;

        Ok(Self { api_key, client })
    }
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
    model: String,
}

#[derive(Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: Option<String>,
}

#[derive(Deserialize)]
struct Usage {
    input_tokens: usize,
    output_tokens: usize,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ErrorDetail,
}

#[derive(Deserialize)]
struct ErrorDetail {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    error_type: Option<String>,
    message: String,
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    fn default_model(&self) -> &str {
        "claude-sonnet-4-20250514"
    }

    async fn complete(&self, prompt: &str, config: &RequestConfig) -> Result<LlmResponse> {
        let model = config
            .model
            .as_deref()
            .unwrap_or_else(|| self.default_model());

        let request = AnthropicRequest {
            model: model.to_string(),
            max_tokens: config.max_tokens,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: Some(config.temperature),
        };

        let response = self
            .client
            .post(ANTHROPIC_API_URL)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error: ErrorResponse = response.json().await?;
            let error_msg = &error.error.message;

            // Check for credit/billing related errors (402 Payment Required or error message)
            if status.as_u16() == 402
                || error_msg.to_lowercase().contains("credit")
                || error_msg.to_lowercase().contains("balance")
                || error_msg.to_lowercase().contains("billing")
            {
                return Err(anyhow!("CREDIT_EXHAUSTED: {}", error_msg));
            }

            return Err(anyhow!("Anthropic API error ({}): {}", status.as_u16(), error_msg));
        }

        let result: AnthropicResponse = response.json().await?;

        let content = result
            .content
            .into_iter()
            .filter_map(|block| {
                if block.content_type == "text" {
                    block.text
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("");

        Ok(LlmResponse {
            content,
            input_tokens: result.usage.input_tokens,
            output_tokens: result.usage.output_tokens,
            model: result.model,
            provider: "anthropic".to_string(),
        })
    }
}
