# resend

A command-line interface for the [Resend](https://resend.com) email platform. Send emails, manage domains, API keys, and templates directly from your terminal.

## Quick Start

```bash
# Install
cargo install --path .

# Configure your API key
resend config setup

# Send an email
resend emails send \
  --from "you@yourdomain.com" \
  --to "recipient@example.com" \
  --subject "Hello from the CLI" \
  --text "Sent via resend-cli!"
```

## Installation

### From Source

Requires [Rust](https://rustup.rs/) 1.70 or later.

```bash
git clone https://github.com/tavva/resend-cli.git
cd resend-cli
cargo install --path .
```

### Build Only

```bash
cargo build --release
# Binary at ./target/release/resend
```

## Configuration

### Interactive Setup

```bash
resend config setup
```

This prompts for your API key, tests the connection, and saves it securely.

### Environment Variable

```bash
export RESEND_API_KEY=re_123456789
```

Environment variables take precedence over config file settings.

### Config File

Configuration is stored in `~/.config/resend/config.yml` (or platform equivalent):

```yaml
profiles:
  default:
    api_key: re_123456789
  production:
    api_key: re_prod_key
```

### Multiple Profiles

```bash
# Use a specific profile
resend emails list --profile production

# Or set via environment
export RESEND_PROFILE=production
```

## Usage

### Emails

```bash
# Send a simple email
resend emails send \
  --from "sender@example.com" \
  --to "recipient@example.com" \
  --subject "Hello" \
  --text "Plain text body"

# Send HTML email
resend emails send \
  --from "sender@example.com" \
  --to "recipient@example.com" \
  --subject "Newsletter" \
  --html "<h1>Welcome</h1><p>Thanks for subscribing!</p>"

# Send to multiple recipients with CC
resend emails send \
  --from "sender@example.com" \
  --to "alice@example.com" \
  --to "bob@example.com" \
  --cc "manager@example.com" \
  --subject "Team Update" \
  --text "Weekly sync notes..."

# Schedule an email
resend emails send \
  --from "sender@example.com" \
  --to "recipient@example.com" \
  --subject "Reminder" \
  --text "Don't forget!" \
  --scheduled-at "2025-01-20T09:00:00Z"

# List recent emails
resend emails list

# Get email details
resend emails get <email-id>

# Cancel a scheduled email
resend emails cancel <email-id>
```

### Domains

```bash
# Add a domain
resend domains create example.com

# Add domain in specific region
resend domains create example.com --region eu-west-1

# List all domains
resend domains list

# Get domain details (includes DNS records)
resend domains get <domain-id>

# Verify domain DNS
resend domains verify <domain-id>

# Update domain settings
resend domains update <domain-id> --open-tracking true --click-tracking true

# Delete a domain
resend domains delete <domain-id>
```

### API Keys

```bash
# Create a new API key
resend api-keys create "My CLI Key"

# Create with restricted permissions
resend api-keys create "Sending Only" --permission sending_access

# Create restricted to specific domain
resend api-keys create "Domain Key" --domain-id <domain-id>

# List all API keys
resend api-keys list

# Delete an API key
resend api-keys delete <key-id>
```

### Templates

```bash
# Create a template
resend templates create "Welcome Email" \
  --subject "Welcome to {{company}}!" \
  --html "<h1>Welcome, {{name}}!</h1>"

# List templates
resend templates list

# Get template details
resend templates get <template-id>

# Update a template
resend templates update <template-id> --subject "New Subject"

# Delete a template
resend templates delete <template-id>
```

## Output Formats

### Table (Default)

Human-readable table output:

```bash
resend emails list
```

```
ID          TO                SUBJECT       STATUS     CREATED
email-123   user@example.com  Hello World   delivered  2025-01-15T10:30:00Z
email-456   test@example.com  Newsletter    sent       2025-01-15T09:00:00Z
```

### JSON

Machine-readable JSON output:

```bash
resend emails list --json
```

```json
[
  {
    "id": "email-123",
    "to": ["user@example.com"],
    "subject": "Hello World",
    "last_event": "delivered",
    "created_at": "2025-01-15T10:30:00Z"
  }
]
```

### Output to File

```bash
resend domains list --json --output domains.json
```

## Commands Reference

| Command | Description |
|---------|-------------|
| `config setup` | Interactive configuration setup |
| `config show` | Display current configuration |
| `config list` | List all profiles |
| `emails send` | Send an email |
| `emails get` | Get email by ID |
| `emails list` | List emails |
| `emails cancel` | Cancel scheduled email |
| `emails update` | Update scheduled email |
| `domains create` | Add a domain |
| `domains list` | List domains |
| `domains get` | Get domain details |
| `domains verify` | Trigger domain verification |
| `domains update` | Update domain settings |
| `domains delete` | Remove a domain |
| `api-keys create` | Create an API key |
| `api-keys list` | List API keys |
| `api-keys delete` | Delete an API key |
| `templates create` | Create a template |
| `templates list` | List templates |
| `templates get` | Get template details |
| `templates update` | Update a template |
| `templates delete` | Delete a template |

## Global Options

These options work with all commands:

| Option | Description |
|--------|-------------|
| `--json` | Output as JSON |
| `--output <FILE>` | Write output to file |
| `--profile <NAME>` | Use specific config profile |
| `--verbose` | Enable verbose output |
| `--help` | Show help |
| `--version` | Show version |

## Development

```bash
# Run tests
cargo test

# Build debug
cargo build

# Build release
cargo build --release

# Run directly
cargo run -- emails list
```

## Licence

MIT
