//! LLM Provider Trait
//!
//! Defines the core abstraction for LLM providers.

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Response from an LLM provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub model: String,
    pub provider: String,
}

/// Configuration for LLM requests
#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub model: Option<String>,
    pub max_tokens: u32,
    pub temperature: f32,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            model: None,
            max_tokens: 8192,
            temperature: 0.0, // Deterministic for reproducibility
        }
    }
}

/// Trait for LLM providers
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Default model for this provider
    fn default_model(&self) -> &str;

    /// Send a completion request
    async fn complete(&self, prompt: &str, config: &RequestConfig) -> Result<LlmResponse>;
}
