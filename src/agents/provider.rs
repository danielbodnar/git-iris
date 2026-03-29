//! Dynamic provider abstraction for rig-core 0.27+
//!
//! This module provides runtime provider selection using enum dispatch,
//! allowing git-iris to work with any supported provider based on config.

use anyhow::Result;
use rig::{
    agent::{Agent, AgentBuilder, PromptResponse},
    client::{CompletionClient, ProviderClient},
    completion::{CompletionModel, Prompt, PromptError},
    providers::{anthropic, gemini, openai},
};
use serde_json::{Map, Value, json};
use std::collections::HashMap;

use crate::providers::{Provider, ProviderConfig};

/// Completion model types for each provider
pub type OpenAIModel = openai::completion::CompletionModel;
pub type AnthropicModel = anthropic::completion::CompletionModel;
pub type GeminiModel = gemini::completion::CompletionModel;

/// Agent builder types for each provider
pub type OpenAIBuilder = AgentBuilder<OpenAIModel>;
pub type AnthropicBuilder = AgentBuilder<AnthropicModel>;
pub type GeminiBuilder = AgentBuilder<GeminiModel>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionProfile {
    MainAgent,
    Subagent,
    StatusMessage,
}

impl CompletionProfile {
    const fn default_openai_reasoning_effort(self) -> &'static str {
        match self {
            Self::MainAgent => "medium",
            Self::Subagent => "low",
            Self::StatusMessage => "none",
        }
    }
}

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
            Self::OpenAI(a) => a.prompt(msg).max_turns(depth).await,
            Self::Anthropic(a) => a.prompt(msg).max_turns(depth).await,
            Self::Gemini(a) => a.prompt(msg).max_turns(depth).await,
        }
    }

    /// Multi-turn prompt with extended details (token usage, etc.)
    pub async fn prompt_extended(
        &self,
        msg: &str,
        depth: usize,
    ) -> Result<PromptResponse, PromptError> {
        match self {
            Self::OpenAI(a) => a.prompt(msg).max_turns(depth).extended_details().await,
            Self::Anthropic(a) => a.prompt(msg).max_turns(depth).extended_details().await,
            Self::Gemini(a) => a.prompt(msg).max_turns(depth).extended_details().await,
        }
    }
}

/// Source of the resolved API key (for logging/debugging)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKeySource {
    Config,
    Environment,
    ClientDefault,
}

/// Validate API key format and log warnings for suspicious keys
fn validate_and_warn(key: &str, provider: Provider, source: &str) {
    if let Err(warning) = provider.validate_api_key_format(key) {
        tracing::warn!(
            provider = %provider,
            source = source,
            "API key format warning: {}",
            warning
        );
    }
}

/// Resolve API key from config or environment variable.
///
/// Resolution order:
/// 1. If `api_key` is `Some` and non-empty, use it (from config)
/// 2. Otherwise, check the provider's environment variable
/// 3. If neither has a key, returns `None` (caller will use `from_env()`)
///
/// Note: An empty string in config is treated as "not configured" and falls
/// back to the environment variable. This allows users to override env vars
/// in config while still supporting env-only setups.
pub fn resolve_api_key(
    api_key: Option<&str>,
    provider: Provider,
) -> (Option<String>, ApiKeySource) {
    // If explicit key provided and non-empty, use it
    if let Some(key) = api_key
        && !key.is_empty()
    {
        tracing::trace!(
            provider = %provider,
            source = "config",
            "Using API key from configuration"
        );
        validate_and_warn(key, provider, "config");
        return (Some(key.to_string()), ApiKeySource::Config);
    }

    // Fall back to environment variable
    if let Ok(key) = std::env::var(provider.api_key_env()) {
        tracing::trace!(
            provider = %provider,
            env_var = %provider.api_key_env(),
            source = "environment",
            "Using API key from environment variable"
        );
        validate_and_warn(&key, provider, "environment");
        return (Some(key), ApiKeySource::Environment);
    }

    tracing::trace!(
        provider = %provider,
        source = "client_default",
        "No API key found, will use client's from_env()"
    );
    (None, ApiKeySource::ClientDefault)
}

/// Create an `OpenAI` agent builder
///
/// # Arguments
/// * `model` - The model name to use
/// * `api_key` - Optional API key from config. Resolution order:
///   1. Non-empty `api_key` parameter (from config)
///   2. `OPENAI_API_KEY` environment variable
///   3. Client's `from_env()` (requires env var to be set)
///
/// # Errors
/// Returns an error if client creation fails (invalid credentials or missing env var).
///
/// # Security
/// Error messages are sanitized to prevent potential API key exposure.
pub fn openai_builder(model: &str, api_key: Option<&str>) -> Result<OpenAIBuilder> {
    let (resolved_key, _source) = resolve_api_key(api_key, Provider::OpenAI);
    let client = match resolved_key {
        Some(key) => openai::Client::new(&key)
            // Sanitize error to prevent potential key exposure in error messages
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to create OpenAI client: authentication or configuration error"
                )
            })?,
        None => openai::Client::from_env(),
    };
    Ok(client.completions_api().agent(model))
}

/// Create an Anthropic agent builder
///
/// # Arguments
/// * `model` - The model name to use
/// * `api_key` - Optional API key from config. Resolution order:
///   1. Non-empty `api_key` parameter (from config)
///   2. `ANTHROPIC_API_KEY` environment variable
///   3. Client's `from_env()` (requires env var to be set)
///
/// # Errors
/// Returns an error if client creation fails (invalid credentials or missing env var).
///
/// # Security
/// Error messages are sanitized to prevent potential API key exposure.
pub fn anthropic_builder(model: &str, api_key: Option<&str>) -> Result<AnthropicBuilder> {
    let (resolved_key, _source) = resolve_api_key(api_key, Provider::Anthropic);
    let client = match resolved_key {
        Some(key) => anthropic::Client::new(&key)
            // Sanitize error to prevent potential key exposure in error messages
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to create Anthropic client: authentication or configuration error"
                )
            })?,
        None => anthropic::Client::from_env(),
    };
    Ok(client.agent(model))
}

/// Create a Gemini agent builder
///
/// # Arguments
/// * `model` - The model name to use
/// * `api_key` - Optional API key from config. Resolution order:
///   1. Non-empty `api_key` parameter (from config)
///   2. `GOOGLE_API_KEY` environment variable
///   3. Client's `from_env()` (requires env var to be set)
///
/// # Errors
/// Returns an error if client creation fails (invalid credentials or missing env var).
///
/// # Security
/// Error messages are sanitized to prevent potential API key exposure.
pub fn gemini_builder(model: &str, api_key: Option<&str>) -> Result<GeminiBuilder> {
    let (resolved_key, _source) = resolve_api_key(api_key, Provider::Google);
    let client = match resolved_key {
        Some(key) => gemini::Client::new(&key)
            // Sanitize error to prevent potential key exposure in error messages
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to create Gemini client: authentication or configuration error"
                )
            })?,
        None => gemini::Client::from_env(),
    };
    Ok(client.agent(model))
}

fn parse_additional_param_value(raw: &str) -> Value {
    serde_json::from_str(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
}

fn additional_params_json(
    additional_params: Option<&HashMap<String, String>>,
) -> Map<String, Value> {
    let mut params = Map::new();
    if let Some(additional_params) = additional_params {
        for (key, value) in additional_params {
            params.insert(key.clone(), parse_additional_param_value(value));
        }
    }
    params
}

fn supports_openai_reasoning_defaults(model: &str) -> bool {
    model.to_lowercase().starts_with("gpt-5")
}

fn completion_params_json(
    additional_params: Option<&HashMap<String, String>>,
    provider: Provider,
    model: &str,
    max_tokens: u64,
    profile: CompletionProfile,
) -> Map<String, Value> {
    let mut params = additional_params_json(additional_params);

    if provider == Provider::OpenAI && needs_max_completion_tokens(model) {
        params.insert("max_completion_tokens".to_string(), json!(max_tokens));
    }

    if provider == Provider::OpenAI
        && supports_openai_reasoning_defaults(model)
        && !params.contains_key("reasoning")
    {
        params.insert(
            "reasoning".to_string(),
            json!({ "effort": profile.default_openai_reasoning_effort() }),
        );
    }

    params
}

fn needs_max_completion_tokens(model: &str) -> bool {
    let model = model.to_lowercase();
    model.starts_with("gpt-5")
        || model.starts_with("gpt-4.1")
        || model.starts_with("o1")
        || model.starts_with("o3")
        || model.starts_with("o4")
}

pub fn apply_completion_params<M>(
    mut builder: AgentBuilder<M>,
    provider: Provider,
    model: &str,
    max_tokens: u64,
    additional_params: Option<&HashMap<String, String>>,
    profile: CompletionProfile,
) -> AgentBuilder<M>
where
    M: CompletionModel,
{
    if !(provider == Provider::OpenAI && needs_max_completion_tokens(model)) {
        builder = builder.max_tokens(max_tokens);
    }

    let params = completion_params_json(additional_params, provider, model, max_tokens, profile);

    if params.is_empty() {
        builder
    } else {
        builder.additional_params(Value::Object(params))
    }
}

pub fn provider_from_name(provider: &str) -> Result<Provider> {
    provider
        .parse()
        .map_err(|_| anyhow::anyhow!("Unsupported provider: {}", provider))
}

pub fn current_provider_config<'a>(
    config: Option<&'a crate::config::Config>,
    provider: &str,
) -> Option<&'a ProviderConfig> {
    config.and_then(|config| config.get_provider_config(provider))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_api_key_uses_config_when_provided() {
        // Config key takes precedence
        let (key, source) = resolve_api_key(Some("sk-config-key-1234567890"), Provider::OpenAI);
        assert_eq!(key, Some("sk-config-key-1234567890".to_string()));
        assert_eq!(source, ApiKeySource::Config);
    }

    #[test]
    fn test_resolve_api_key_empty_config_not_used() {
        // Empty config should NOT be treated as a valid key
        // It should fall through to env var or client default
        let empty_config: Option<&str> = Some("");
        let (_key, source) = resolve_api_key(empty_config, Provider::OpenAI);

        // Empty config should NOT return Config source
        // This test verifies the empty string is treated as "not configured"
        assert_ne!(source, ApiKeySource::Config);
    }

    #[test]
    fn test_resolve_api_key_none_config_checks_env() {
        // When config is None, should check env var
        let (key, source) = resolve_api_key(None, Provider::OpenAI);

        // Result depends on whether OPENAI_API_KEY is set in the environment
        // We just verify the function doesn't panic and returns appropriate source
        match source {
            ApiKeySource::Environment => {
                assert!(key.is_some());
            }
            ApiKeySource::ClientDefault => {
                assert!(key.is_none());
            }
            ApiKeySource::Config => {
                unreachable!("Should not return Config source when config is None");
            }
        }
    }

    #[test]
    fn test_api_key_source_enum_equality() {
        assert_eq!(ApiKeySource::Config, ApiKeySource::Config);
        assert_eq!(ApiKeySource::Environment, ApiKeySource::Environment);
        assert_eq!(ApiKeySource::ClientDefault, ApiKeySource::ClientDefault);
        assert_ne!(ApiKeySource::Config, ApiKeySource::Environment);
    }

    #[test]
    fn test_resolve_api_key_all_providers() {
        // Test that resolve_api_key works for all supported providers
        for provider in Provider::ALL {
            let (key, source) = resolve_api_key(Some("test-key-123456789012345"), *provider);
            assert_eq!(key, Some("test-key-123456789012345".to_string()));
            assert_eq!(source, ApiKeySource::Config);
        }
    }

    #[test]
    fn test_resolve_api_key_config_precedence() {
        // Even if env var is set, config should take precedence
        // We can't easily mock env vars in unit tests, but we can verify
        // that a provided config key is always used regardless of env state
        let config_key = "sk-from-config-abcdef1234567890";
        let (key, source) = resolve_api_key(Some(config_key), Provider::OpenAI);

        assert_eq!(key.as_deref(), Some(config_key));
        assert_eq!(source, ApiKeySource::Config);
    }

    #[test]
    fn test_api_key_source_debug_impl() {
        // Verify Debug is implemented for logging purposes
        let source = ApiKeySource::Config;
        let debug_str = format!("{:?}", source);
        assert!(debug_str.contains("Config"));
    }

    #[test]
    fn test_apply_completion_params_parses_json_like_additional_params() {
        let mut additional_params = HashMap::new();
        additional_params.insert("temperature".to_string(), "0.7".to_string());
        additional_params.insert("reasoning".to_string(), r#"{"effort":"low"}"#.to_string());

        let params = additional_params_json(Some(&additional_params));
        assert_eq!(params.get("temperature"), Some(&json!(0.7)));
        assert_eq!(params.get("reasoning"), Some(&json!({"effort": "low"})));
    }

    #[test]
    fn test_completion_params_use_profile_specific_openai_reasoning_defaults() {
        let main_params = completion_params_json(
            None,
            Provider::OpenAI,
            "gpt-5.4",
            16_384,
            CompletionProfile::MainAgent,
        );
        assert_eq!(
            main_params.get("reasoning"),
            Some(&json!({"effort": "medium"}))
        );
        assert_eq!(
            main_params.get("max_completion_tokens"),
            Some(&json!(16_384))
        );

        let status_params = completion_params_json(
            None,
            Provider::OpenAI,
            "gpt-5.4-mini",
            50,
            CompletionProfile::StatusMessage,
        );
        assert_eq!(
            status_params.get("reasoning"),
            Some(&json!({"effort": "none"}))
        );
        assert_eq!(status_params.get("max_completion_tokens"), Some(&json!(50)));
    }

    #[test]
    fn test_completion_params_preserve_explicit_reasoning_overrides() {
        let mut additional_params = HashMap::new();
        additional_params.insert("reasoning".to_string(), r#"{"effort":"high"}"#.to_string());

        let params = completion_params_json(
            Some(&additional_params),
            Provider::OpenAI,
            "gpt-5.4",
            4096,
            CompletionProfile::MainAgent,
        );

        assert_eq!(params.get("reasoning"), Some(&json!({"effort": "high"})));
    }

    #[test]
    fn test_completion_params_skip_openai_reasoning_defaults_for_non_gpt5_models() {
        let params = completion_params_json(
            None,
            Provider::OpenAI,
            "gpt-4.1",
            4096,
            CompletionProfile::MainAgent,
        );

        assert!(!params.contains_key("reasoning"));
        assert_eq!(params.get("max_completion_tokens"), Some(&json!(4096)));
    }

    #[test]
    fn test_provider_from_name_supports_aliases() {
        assert_eq!(provider_from_name("openai").ok(), Some(Provider::OpenAI));
        assert_eq!(provider_from_name("claude").ok(), Some(Provider::Anthropic));
        assert_eq!(provider_from_name("gemini").ok(), Some(Provider::Google));
    }

    #[test]
    fn test_needs_max_completion_tokens_for_gpt5_family() {
        assert!(needs_max_completion_tokens("gpt-5.4"));
        assert!(needs_max_completion_tokens("o3"));
        assert!(!needs_max_completion_tokens("claude-opus-4-6"));
    }
}
