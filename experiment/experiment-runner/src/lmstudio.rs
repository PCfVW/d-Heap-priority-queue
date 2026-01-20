//! LM Studio Provider (OpenAI-compatible local server)

use crate::provider::{LlmProvider, LlmResponse, RequestConfig};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct LmStudioProvider {
    base_url: String,
    client: reqwest::Client,
}

impl LmStudioProvider {
    pub fn new() -> Self {
        let base_url = std::env::var("LMSTUDIO_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:1234/v1".to_string());

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(600)) // 10 min for local models
            .build()
            .unwrap_or_default();

        Self { base_url, client }
    }

    /// Check if LM Studio is running and get the loaded model
    pub async fn get_loaded_model(&self) -> Result<String> {
        let url = format!("{}/models", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("LM Studio not reachable at {}", self.base_url));
        }

        let models: ModelsResponse = response.json().await?;
        models
            .data
            .first()
            .map(|m| m.id.clone())
            .ok_or_else(|| anyhow!("No model loaded in LM Studio"))
    }
}

// OpenAI-compatible request/response format (same as Mistral)
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
    #[serde(default)]
    usage: Option<ChatUsage>,
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

#[derive(Deserialize)]
struct ModelsResponse {
    data: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    id: String,
}

#[async_trait]
impl LlmProvider for LmStudioProvider {
    fn default_model(&self) -> &str {
        "loaded-model" // Will be replaced with actual model from /models endpoint
    }

    async fn complete(&self, prompt: &str, config: &RequestConfig) -> Result<LlmResponse> {
        // Get the actual loaded model name
        let model = match &config.model {
            Some(m) => m.clone(),
            None => self.get_loaded_model().await?,
        };

        let request = ChatRequest {
            model: model.clone(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            max_tokens: config.max_tokens,
            temperature: config.temperature,
        };

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("LM Studio error ({}): {}", status, error_text));
        }

        let result: ChatResponse = response.json().await?;

        let content = result
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default();

        // LM Studio may not always return usage stats
        let (input_tokens, output_tokens) = result
            .usage
            .map(|u| (u.prompt_tokens, u.completion_tokens))
            .unwrap_or((0, 0));

        Ok(LlmResponse {
            content,
            input_tokens,
            output_tokens,
            model: result.model,
            provider: "lmstudio".to_string(),
        })
    }
}
