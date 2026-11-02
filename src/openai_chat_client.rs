use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::chat_config::ChatConfig;
use crate::conversation::ChatMessage;

pub(crate) struct OpenAiChatClient {
    http_client: Client,
    api_key: String,
    api_url: String,
    model: String,
}

#[derive(Debug, Error)]
pub(crate) enum OpenAiChatError {
    #[error("Chat API request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Chat API returned HTTP {status}: {response_body}")]
    ApiResponse {
        status: StatusCode,
        response_body: String,
    },
    #[error("Chat API response did not contain a completion choice")]
    MissingCompletionChoice,
    #[error("Chat API returned an empty assistant message")]
    EmptyAssistantMessage,
}

#[derive(Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
}

#[derive(Deserialize)]
struct ChatCompletionResponse {
    choices: Vec<ChatCompletionChoice>,
}

#[derive(Deserialize)]
struct ChatCompletionChoice {
    message: ChatMessage,
}

impl OpenAiChatClient {
    pub(crate) fn new(config: &ChatConfig) -> Self {
        Self {
            http_client: Client::new(),
            api_key: config.api_key.clone(),
            api_url: config.api_url.clone(),
            model: config.model.clone(),
        }
    }

    pub(crate) async fn create_chat_completion(
        &self,
        messages: &[ChatMessage],
    ) -> Result<String, OpenAiChatError> {
        let request = ChatCompletionRequest {
            model: &self.model,
            messages,
        };
        let response = self
            .http_client
            .post(&self.api_url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await?;
        let status = response.status();

        if !status.is_success() {
            let response_body = response.text().await?;
            return Err(OpenAiChatError::ApiResponse {
                status,
                response_body,
            });
        }

        let response: ChatCompletionResponse = response.json().await?;
        let assistant_message = response
            .choices
            .into_iter()
            .next()
            .ok_or(OpenAiChatError::MissingCompletionChoice)?
            .message
            .content;

        if assistant_message.trim().is_empty() {
            return Err(OpenAiChatError::EmptyAssistantMessage);
        }

        Ok(assistant_message)
    }
}
