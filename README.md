# Rust AI Chatbot

A multi-turn terminal chatbot written in Rust. It works with OpenAI and other providers that expose an OpenAI-compatible `/chat/completions` endpoint.

## Features

- Keeps conversation history for follow-up questions
- Supports a configurable model, endpoint, and system prompt
- Preserves the system prompt when the conversation is cleared
- Uses Rustls, so OpenSSL is not required

## Run it

1. Create your local environment file:

   ```bash
   cp .env.example .env
   ```

2. Put your API key in `.env`:

   ```dotenv
   OPENAI_API_KEY=your-api-key-here
   ```

3. Start the chatbot:

   ```bash
   cargo run
   ```

## Commands

- `/help` — list commands
- `/clear` — clear conversation history
- `/exit` or `/quit` — close the chatbot

## Configuration

| Variable | Required | Default |
| --- | --- | --- |
| `OPENAI_API_KEY` | Yes | None |
| `AI_API_URL` | No | `https://api.openai.com/v1/chat/completions` |
| `AI_MODEL` | No | `gpt-4o-mini` |
| `AI_SYSTEM_PROMPT` | No | `You are a helpful, concise assistant.` |

For a local OpenAI-compatible server, point `AI_API_URL` at its chat completions endpoint and set `AI_MODEL` to a model it serves. The server must accept bearer authentication; use any non-empty key if your local server ignores the token.

## Verify

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
