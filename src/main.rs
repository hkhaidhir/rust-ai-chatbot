mod ai_chat_client;
mod chat_config;
mod conversation;
mod terminal_chat;

use std::error::Error;
use std::io::{self, Write};

use ai_chat_client::AiChatClient;
use chat_config::ChatConfig;
use conversation::ChatConversation;
use terminal_chat::{TerminalChatAction, parse_terminal_chat_action};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let config = ChatConfig::from_environment()?;
    let chat_client = AiChatClient::new(&config);
    let mut conversation = ChatConversation::new(config.system_prompt);

    println!("Rust AI Chatbot ({})", config.model);
    print_terminal_help();

    loop {
        let Some(input) = read_terminal_input()? else {
            println!("\nGoodbye!");
            break;
        };

        match parse_terminal_chat_action(&input) {
            TerminalChatAction::Exit => {
                println!("Goodbye!");
                break;
            }
            TerminalChatAction::Clear => {
                conversation.clear_chat_history();
                println!("Conversation cleared.");
            }
            TerminalChatAction::Help => print_terminal_help(),
            TerminalChatAction::Empty => {}
            TerminalChatAction::UnknownCommand(command) => {
                println!("Unknown command: {command}. Type /help to list commands.");
            }
            TerminalChatAction::SendMessage(message) => {
                conversation.add_user_message(message);
                print!("AI: ");
                io::stdout().flush()?;

                match chat_client
                    .create_chat_completion(conversation.messages())
                    .await
                {
                    Ok(response) => {
                        println!("{response}\n");
                        conversation.add_assistant_message(response);
                    }
                    Err(error) => {
                        conversation.remove_latest_user_message();
                        eprintln!("\nCould not get an AI response: {error}\n");
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_terminal_input() -> io::Result<Option<String>> {
    print!("You: ");
    io::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    Ok(Some(input))
}

fn print_terminal_help() {
    println!("Commands: /help, /clear, /exit\n");
}
