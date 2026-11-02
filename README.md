# Rust AI Chatbot

A provider-neutral, multi-turn terminal chatbot written in Rust. It works with AI providers that expose an OpenAI-compatible `/chat/completions` endpoint and uses Gemini by default.

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

2. Create a Gemini API key in [Google AI Studio](https://aistudio.google.com/app/apikey), then put it in `.env`:

   ```dotenv
   AI_API_KEY=your-gemini-api-key-here
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
| `AI_API_KEY` | Yes | None |
| `AI_API_URL` | No | `https://generativelanguage.googleapis.com/v1beta/openai/chat/completions` |
| `AI_MODEL` | No | `gemini-2.5-flash` |
| `AI_SYSTEM_PROMPT` | No | `You are a helpful, concise assistant.` |

The default model is subject to Gemini's free-tier quotas and regional availability. To use another OpenAI-compatible provider, change `AI_API_URL`, `AI_MODEL`, and `AI_API_KEY` to the values required by that provider.

## Verify

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
