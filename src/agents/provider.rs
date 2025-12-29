//! Dynamic provider abstraction for rig-core 0.27+
//!
//! This module provides runtime provider selection using enum dispatch,
//! allowing git-iris to work with any supported provider based on config.

use rig::{
    agent::{Agent, AgentBuilder, PromptResponse},
    client::{CompletionClient, ProviderClient},
    completion::{Prompt, PromptError},
    providers::{anthropic, gemini, openai},
};

/// Completion model types for each provider
pub type OpenAIModel = openai::completion::CompletionModel;
pub type AnthropicModel = anthropic::completion::CompletionModel;
pub type GeminiModel = gemini::completion::CompletionModel;

/// Agent builder types for each provider
pub type OpenAIBuilder = AgentBuilder<OpenAIModel>;
pub type AnthropicBuilder = AgentBuilder<AnthropicModel>;
pub type GeminiBuilder = AgentBuilder<GeminiModel>;

/// Dynamic agent that can be any provider's agent type
pub enum DynAgent {
    OpenAI(Agent<OpenAIModel>),
    Anthropic(Agent<AnthropicModel>),
    Gemini(Agent<GeminiModel>),
}

impl DynAgent {
    /// Simple prompt - returns response string
    pub async fn prompt(&self, msg: &str) -> Result<String, PromptError> {
        match self {
            Self::OpenAI(a) => a.prompt(msg).await,
            Self::Anthropic(a) => a.prompt(msg).await,
            Self::Gemini(a) => a.prompt(msg).await,
        }
    }

    /// Multi-turn prompt with specified depth for tool calling
    pub async fn prompt_multi_turn(&self, msg: &str, depth: usize) -> Result<String, PromptError> {
        match self {
            Self::OpenAI(a) => a.prompt(msg).multi_turn(depth).await,
            Self::Anthropic(a) => a.prompt(msg).multi_turn(depth).await,
            Self::Gemini(a) => a.prompt(msg).multi_turn(depth).await,
        }
    }

    /// Multi-turn prompt with extended details (token usage, etc.)
    pub async fn prompt_extended(
        &self,
        msg: &str,
        depth: usize,
    ) -> Result<PromptResponse, PromptError> {
        match self {
            Self::OpenAI(a) => a.prompt(msg).multi_turn(depth).extended_details().await,
            Self::Anthropic(a) => a.prompt(msg).multi_turn(depth).extended_details().await,
            Self::Gemini(a) => a.prompt(msg).multi_turn(depth).extended_details().await,
        }
    }
}

/// Create an `OpenAI` agent builder
pub fn openai_builder(model: &str) -> OpenAIBuilder {
    let client = openai::Client::from_env();
    client.completions_api().agent(model)
}

/// Create an Anthropic agent builder
pub fn anthropic_builder(model: &str) -> AnthropicBuilder {
    let client = anthropic::Client::from_env();
    client.agent(model)
}

/// Create a Gemini agent builder
pub fn gemini_builder(model: &str) -> GeminiBuilder {
    let client = gemini::Client::from_env();
    client.agent(model)
}
