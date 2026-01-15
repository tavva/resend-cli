# Resend CLI Design

A Rust CLI for the Resend email platform, designed for use with Claude Code skills.

## Goals

- Full platform management from the terminal
- SDK-style command structure (`resend emails send` matches `resend.emails.send`)
- Human-readable output by default, JSON for scripting
- Consistent patterns with the `lf` CLI

## Project Structure

```
resend-cli/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, clap CLI dispatch
│   ├── client.rs         # Resend API client with typed methods
│   ├── config.rs         # Config file (YAML), profiles, env var priority
│   ├── types.rs          # Request/response types for Resend API
│   ├── formatters/
│   │   ├── mod.rs
│   │   ├── table.rs      # Human-readable tables
│   │   └── json.rs       # JSON output
│   └── commands/
│       ├── mod.rs        # build_config(), format_and_output() helpers
│       ├── config.rs     # config setup, config show, config list
│       ├── emails.rs     # emails send, get, list, cancel, update
│       ├── domains.rs    # domains create, list, get, verify, update, delete
│       ├── api_keys.rs   # api-keys create, list, delete
│       └── templates.rs  # templates create, list, get, update, delete
```

## Dependencies

```toml
[dependencies]
clap = { version = "4", features = ["derive", "env"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yaml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
tabled = "0.16"
directories = "5"
dirs = "5"
thiserror = "2"
anyhow = "1"
dialoguer = "0.11"
dotenvy = "0.15"

[dev-dependencies]
wiremock = "0.6"
tempfile = "3"
```

## Commands

### emails

- `resend emails send` - Send an email
  - Required: `--from`, `--to`, `--subject`
  - Optional: `--html`, `--text`, `--cc`, `--bcc`, `--reply-to`, `--headers`, `--attachments`, `--tags`, `--scheduled-at`
- `resend emails get <id>` - Get email by ID
- `resend emails list` - List emails (optional: `--limit`)
- `resend emails cancel <id>` - Cancel scheduled email
- `resend emails update <id>` - Update scheduled email (`--scheduled-at`)

### domains

- `resend domains create <name>` - Add domain (optional: `--region`)
- `resend domains list` - List all domains
- `resend domains get <id>` - Get domain details
- `resend domains verify <id>` - Trigger DNS verification
- `resend domains update <id>` - Update settings (`--click-tracking`, `--open-tracking`, `--tls`)
- `resend domains delete <id>` - Remove domain

### api-keys

- `resend api-keys create <name>` - Create key (optional: `--permission`, `--domain-id`)
- `resend api-keys list` - List all keys
- `resend api-keys delete <id>` - Delete key

### templates

- `resend templates create <name>` - Create template (`--subject`, `--html` or `--text`)
- `resend templates list` - List templates
- `resend templates get <id>` - Get template
- `resend templates update <id>` - Update template
- `resend templates delete <id>` - Remove template

### config

- `resend config setup` - Interactive setup
- `resend config show` - Show current config
- `resend config list` - List profiles

### Global flags

- `--json` - Output as JSON
- `--profile <name>` - Use specific profile
- `-v, --verbose` - Show debug info
- `-o, --output <file>` - Write to file

## Configuration

### File location

- Primary: `~/.config/resend/config.yml`
- Fallback: `~/.resend/config.yml`

### Format

```yaml
profiles:
  default:
    api_key: re_xxxxxxxxx
  production:
    api_key: re_yyyyyyyyy
```

### Resolution priority

1. Environment variable: `RESEND_API_KEY`
2. Config file profile

### Security

- Config file permissions set to `0600` on Unix
- No special handling on Windows (user profile directory is protected)

## Output

### Human-readable (default)

```
$ resend emails list
ID                                    TO                     SUBJECT           STATUS      CREATED
550e8400-e29b-41d4-a716-446655440000  user@example.com       Welcome!          delivered   2025-01-15 10:30
```

### JSON (`--json`)

```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "to": ["user@example.com"],
    "subject": "Welcome!",
    "status": "delivered",
    "created_at": "2025-01-15T10:30:00Z"
  }
]
```

### Errors (always JSON to stderr)

```json
{"error": "authentication_failed", "message": "Invalid API key"}
```

## API Client

Base URL: `https://api.resend.com`

Authentication: Bearer token in Authorization header

### Error types

- `AuthenticationError` - Invalid API key
- `NotFoundError` - Resource not found
- `RateLimitError` - Rate limit exceeded
- `ValidationError` - Invalid request
- `ApiError` - Other API errors
- `NetworkError` - Connection issues

## Testing

- `wiremock` for HTTP mocking
- `tempfile` for config file tests
- Coverage: client methods, config handling, output formatters

## Scope

Initial release covers core features:
- Emails (send, get, list, cancel, update)
- Domains (CRUD, verify)
- API Keys (CRUD)
- Templates (CRUD)

Future additions:
- Contacts & Audiences
- Broadcasts
- Webhooks
- Segments
