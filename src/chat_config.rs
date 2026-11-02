use std::env;

use thiserror::Error;

const DEFAULT_AI_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions";
const DEFAULT_AI_MODEL: &str = "gemini-3.5-flash-lite";
const DEFAULT_SYSTEM_PROMPT: &str = "You are a helpful, concise assistant.";

#[derive(Debug, Clone)]
pub(crate) struct ChatConfig {
    pub(crate) api_key: String,
    pub(crate) api_url: String,
    pub(crate) model: String,
    pub(crate) system_prompt: String,
}

#[derive(Debug, Error)]
pub(crate) enum ChatConfigError {
    #[error("Missing AI_API_KEY. Copy .env.example to .env and add your provider API key.")]
    MissingAiApiKey,
}

impl ChatConfig {
    pub(crate) fn from_environment() -> Result<Self, ChatConfigError> {
        let api_key = env::var("AI_API_KEY")
            .ok()
            .filter(|value| !value.trim().is_empty())
            .ok_or(ChatConfigError::MissingAiApiKey)?;

        Ok(Self {
            api_key,
            api_url: environment_value_or_default("AI_API_URL", DEFAULT_AI_API_URL),
            model: environment_value_or_default("AI_MODEL", DEFAULT_AI_MODEL),
            system_prompt: environment_value_or_default("AI_SYSTEM_PROMPT", DEFAULT_SYSTEM_PROMPT),
        })
    }
}

fn environment_value_or_default(variable_name: &str, default_value: &str) -> String {
    env::var(variable_name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| default_value.to_owned())
}
