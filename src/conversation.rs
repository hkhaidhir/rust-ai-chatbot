use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ChatRole {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub(crate) struct ChatMessage {
    pub(crate) role: ChatRole,
    pub(crate) content: String,
}

impl ChatMessage {
    pub(crate) fn system(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::System,
            content: content.into(),
        }
    }

    pub(crate) fn user(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::User,
            content: content.into(),
        }
    }

    pub(crate) fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: ChatRole::Assistant,
            content: content.into(),
        }
    }
}

pub(crate) struct ChatConversation {
    system_prompt: String,
    messages: Vec<ChatMessage>,
}

impl ChatConversation {
    pub(crate) fn new(system_prompt: impl Into<String>) -> Self {
        let system_prompt = system_prompt.into();
        let messages = vec![ChatMessage::system(system_prompt.clone())];

        Self {
            system_prompt,
            messages,
        }
    }

    pub(crate) fn add_user_message(&mut self, content: impl Into<String>) {
        self.messages.push(ChatMessage::user(content));
    }

    pub(crate) fn add_assistant_message(&mut self, content: impl Into<String>) {
        self.messages.push(ChatMessage::assistant(content));
    }

    pub(crate) fn clear_chat_history(&mut self) {
        self.messages = vec![ChatMessage::system(self.system_prompt.clone())];
    }

    pub(crate) fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    pub(crate) fn remove_latest_user_message(&mut self) {
        if self.messages.last().map(|message| message.role) == Some(ChatRole::User) {
            self.messages.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ChatConversation, ChatMessage};

    #[test]
    fn clear_chat_history_preserves_system_prompt() {
        let mut conversation = ChatConversation::new("Be helpful.");
        conversation.add_user_message("Hello");
        conversation.add_assistant_message("Hi!");

        conversation.clear_chat_history();

        assert_eq!(
            conversation.messages(),
            &[ChatMessage::system("Be helpful.")]
        );
    }

    #[test]
    fn remove_latest_user_message_only_removes_user_role() {
        let mut conversation = ChatConversation::new("Be helpful.");
        conversation.add_user_message("Will this fail?");

        conversation.remove_latest_user_message();
        conversation.remove_latest_user_message();

        assert_eq!(
            conversation.messages(),
            &[ChatMessage::system("Be helpful.")]
        );
    }
}
