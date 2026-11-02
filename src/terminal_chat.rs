#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TerminalChatAction {
    Exit,
    Clear,
    Help,
    Empty,
    UnknownCommand(String),
    SendMessage(String),
}

pub(crate) fn parse_terminal_chat_action(input: &str) -> TerminalChatAction {
    let trimmed_input = input.trim();

    match trimmed_input {
        "" => TerminalChatAction::Empty,
        "/exit" | "/quit" => TerminalChatAction::Exit,
        "/clear" => TerminalChatAction::Clear,
        "/help" => TerminalChatAction::Help,
        command if command.starts_with('/') => {
            TerminalChatAction::UnknownCommand(command.to_owned())
        }
        message => TerminalChatAction::SendMessage(message.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::{TerminalChatAction, parse_terminal_chat_action};

    #[test]
    fn parses_supported_terminal_commands() {
        assert_eq!(
            parse_terminal_chat_action(" /exit "),
            TerminalChatAction::Exit
        );
        assert_eq!(
            parse_terminal_chat_action("/clear"),
            TerminalChatAction::Clear
        );
        assert_eq!(
            parse_terminal_chat_action("/help"),
            TerminalChatAction::Help
        );
    }

    #[test]
    fn preserves_trimmed_message_text() {
        assert_eq!(
            parse_terminal_chat_action("  Hello, AI!  "),
            TerminalChatAction::SendMessage("Hello, AI!".to_owned())
        );
    }

    #[test]
    fn identifies_unknown_terminal_commands() {
        assert_eq!(
            parse_terminal_chat_action("/restart"),
            TerminalChatAction::UnknownCommand("/restart".to_owned())
        );
    }
}
