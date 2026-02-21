# discli - Discord Notifications CLI

A simple command-line tool for sending Discord notifications programmatically. `discli` enables you to send messages to any Discord channel via a bot with a single command, making it perfect for CI/CD pipelines, automated monitoring, and system alerts.

## Features

- 🚀 **Simple CLI Interface**: Send messages with a single command
- 📸 **Image Support**: Attach images to your Discord messages
- ⚙️ **Environment-based Configuration**: Secure token management via `.env` file
- 📡 **Async Operations**: Built on Tokio for efficient HTTP requests
- 🔒 **Secure**: No hardcoded credentials - all configuration via environment variables
- 🎯 **Lightweight**: Minimal dependencies and small binary size
- 🪝 **Hooks System**: Listen mode that responds to Discord messages with custom commands and automations

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
- [Hooks System](#hooks-system)
  - [Overview](#overview)
  - [Quick Start](#quick-start)
  - [Configuration File](#configuration-file)
  - [Trigger Types](#trigger-types)
  - [Processor Types](#processor-types)
  - [Actions](#actions)
  - [Prompt Templates](#prompt-templates)
  - [Environment Variables](#environment-variables-1)
  - [Example Configurations](#example-configurations)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Advanced Examples](#advanced-examples)
  - [CI/CD Integration](#cicd-integration)
  - [Monitoring & Alerts](#monitoring--alerts)
- [API Reference](#api-reference)
- [Dependencies](#dependencies)
- [Troubleshooting](#troubleshooting)
- [License](#license)

## Prerequisites

Before installing `discli`, ensure you have:

- **Rust Toolchain**: Rust 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Discord Bot**: A Discord application with bot capabilities
- **Bot Token**: A valid Discord bot token with message sending permissions
- **Channel ID**: The Discord channel ID where messages will be sent

### Creating a Discord Bot

1. Go to [Discord Developer Portal](https://discord.com/developers/applications)
2. Click "New Application" and give it a name
3. Navigate to the "Bot" tab and click "Add Bot"
4. Copy the bot token (you'll need this for configuration)
5. Under "Privileged Gateway Intents", enable "Message Content Intent" if needed
6. Invite the bot to your server with `Send Messages` permission

### Finding Your Channel ID

1. Enable Developer Mode in Discord (User Settings → Advanced → Developer Mode)
2. Right-click on the target channel
3. Select "Copy ID" from the context menu

## Installation

### From Source

Clone this repository and build the project:

```bash
git clone <repository-url>
cd discord-notifications
cargo build --release
```

The compiled binary will be available at `target/release/discli`.

### Install Globally (Optional)

To install `discli` globally and use it from anywhere:

```bash
cargo install --path .
```

This installs the binary to `~/.cargo/bin/discli` (on Unix) or `%USERPROFILE%\.cargo\bin\discli.exe` (on Windows).

### Add to PATH (Linux/macOS)

If the binary isn't automatically in your PATH:

```bash
export PATH="$PATH:$HOME/.cargo/bin"
```

Add this line to your `~/.bashrc` or `~/.zshrc` to make it permanent.

### Add to PATH (Windows)

Add `C:\Users\YourUsername\.cargo\bin` to your PATH environment variable.

## Configuration

`discli` requires configuration through a `discli.env` file in the project directory (or environment variables).

### Setup Instructions

1. Copy the example environment file:

```bash
cp discli.env.example discli.env
```

2. Edit `discli.env` with your credentials:

```env
# Discord Bot Configuration
# Your Discord bot token
DISCORD_TOKEN=your_actual_discord_bot_token_here

# Discord channel ID where messages will be sent
DISCORD_CHANNEL_ID=123456789012345678
```

### Security Notes

- ⚠️ **Never commit `discli.env` to version control**
- Add `discli.env` to your `.gitignore` file
- The `.gitignore` in this repository already excludes `discli.env`
- Only share the bot token with trusted individuals

### Alternative Configuration

Instead of using `discli.env`, you can set environment variables directly:

```bash
export DISCORD_TOKEN="your_bot_token"
export DISCORD_CHANNEL_ID="123456789012345678"
discli "Hello, Discord!"
```

## Hooks System

The hooks system enables `discli` to listen to Discord channels and respond to messages automatically. When running in listen mode, the bot monitors configured channels and executes actions when messages match defined triggers.

### Overview

The hooks system provides:

- **Listen Mode**: Monitor Discord channels for matching messages
- **Trigger Matching**: Match messages by prefix, content, regex, or mentions
- **Command Processor**: Execute shell commands and return the output
- **HTTP Processor**: Send messages to webhook endpoints for external processing
- **Flexible Actions**: Reply in channel, send DM, forward to another channel, or call webhooks
- **Rate Limiting**: Prevent abuse with configurable rate limits
- **Prompt Templates**: Process messages using customizable templates

### Quick Start

1. Copy the example hooks configuration:

```bash
cp hooks.yaml.example hooks.yaml
```

2. Edit `hooks.yaml` to configure your hooks (see [Configuration File](#configuration-file) below)

3. Ensure you have a Discord bot token in your `.env` file:

```env
DISCORD_TOKEN=your_bot_token_here
```

4. Start the listener:

```bash
discli listen
```

5. Send a message that matches your trigger (e.g., `!echo hello`) in the configured channel

### Configuration File

The hooks system uses a YAML configuration file (default: `hooks.yaml`). Here's the complete structure:

```yaml
version: "1.0"

# Global settings
settings:
  on_error: "log"           # Strategy when errors occur: log, ignore, notify
  rate_limit:
    per_user: 5              # Max triggers per user in window
    per_channel: 10         # Max triggers per channel in window
    window_seconds: 60      # Rate limit window in seconds

# Prompt templates directory
prompts_dir: "./prompts"

# Hook definitions
hooks:
  - id: "unique-hook-id"
    name: "Human Readable Name"
    enabled: true
    
    # Channel IDs to listen on (as strings)
    channels:
      - "123456789012345678"
    
    # Trigger configuration (see Trigger Types below)
    trigger:
      type: "prefix"
      prefix: "!echo"
    
    # Path to prompt file (relative to prompts_dir)
    prompt_file: "simple-echo.txt"
    
    # Optional filter for specific users/roles
    filter:
      users: []
      roles: []
    
    # Action to take when hook triggers
    action:
      type: "reply"
    
    # Processing configuration
    processing:
      timeout_seconds: 30
      processor_type: "command"
      cmd: ["python", "-c", "import sys; print(sys.stdin.read())"]
```

### Trigger Types

The hooks system supports the following trigger types:

| Type | Description | Configuration |
|------|-------------|----------------|
| `prefix` | Trigger when message starts with a specific prefix | `prefix: "!"` |
| `contains` | Trigger when message contains a substring | `substring: "hello"` |
| `regex` | Trigger when message matches a regex pattern | `pattern: "(?i)(help|support)"` |
| `mention` | Trigger when bot is mentioned | (no additional config) |
| `any` | Trigger on every message | (no additional config) |

#### Prefix Trigger

Triggers when the message starts with a specific prefix (like a command):

```yaml
trigger:
  type: "prefix"
  prefix: "!echo"
```

Matches: `!echo hello`, `!echo test`, `!echo`

#### Contains Trigger

Triggers when the message contains a specific substring:

```yaml
trigger:
  type: "contains"
  substring: "help"
```

Matches: "Can someone help me?", "help needed", "please help"

#### Regex Trigger

Triggers when the message matches a regex pattern:

```yaml
trigger:
  type: "regex"
  pattern: "^!\\w+.*"
```

Matches any message starting with `!` followed by word characters.

#### Mention Trigger

Triggers when the bot is mentioned in a message:

```yaml
trigger:
  type: "mention"
```

### Processor Types

Processors determine how the matched message is processed and what response is generated.

#### Command Processor

Executes a shell command and returns its stdout. The message content is passed to stdin.

```yaml
processing:
  processor_type: "command"
  cmd: ["python", "-c", "import sys; print(sys.stdin.read())"]
  timeout_seconds: 30
```

**Example - Echo Command:**
```yaml
processing:
  processor_type: "command"
  cmd: ["echo", "You said: "]
  # stdin receives the processed prompt
```

**Example - Python Script:**
```yaml
processing:
  processor_type: "command"
  cmd: ["python", "scripts/my_processor.py"]
  timeout_seconds: 30
```

#### HTTP Processor

Sends the processed prompt to an HTTP endpoint and returns the response.

```yaml
processing:
  processor_type: "http"
  url: "https://api.example.com/process"
  timeout_seconds: 30
```

The HTTP processor sends a POST request with JSON body:

```json
{
  "prompt": "The processed prompt content",
  "metadata": {
    "author_name": "username",
    "author_id": "123456789",
    "channel_id": "123456789012345678",
    "message_id": "123456789012345678",
    "timestamp": "2024-01-15T10:30:00Z"
  }
}
```

### Actions

Actions define what happens after processing completes:

| Type | Description | Configuration |
|------|-------------|----------------|
| `reply` | Send a reply in the same channel | (no additional config) |
| `send_dm` | Send a direct message to the user | (no additional config) |
| `forward` | Forward to another channel | `channel_id: "123456789"` |
| `webhook` | Send to a webhook URL | `url: "https://..."` |

```yaml
# Reply in channel
action:
  type: "reply"

# Send DM to user
action:
  type: "send_dm"

# Forward to another channel
action:
  type: "forward"
  channel_id: "987654321098765432"

# Send to webhook
action:
  type: "webhook"
  url: "https://discord.com/api/webhooks/..."
```

### Prompt Templates

Prompt templates transform the incoming message before processing. They support variable substitution:

| Variable | Description |
|----------|-------------|
| `{{author_name}}` | Username of the message author |
| `{{author_id}}` | Discord ID of the author |
| `{{channel_id}}` | Channel ID where message was sent |
| `{{message_id}}` | Message ID |
| `{{content}}` | Raw message content |
| `{{timestamp}}` | Message timestamp (ISO 8601) |
| `{{attachments}}` | List of attachment URLs |

#### Example Prompt Template

Create a file in your prompts directory (e.g., `prompts/simple-echo.txt`):

```
Echo from {{author_name}}:
{{content}}
```

When a message "hello" is received, the processed prompt becomes:

```
Echo from MyUser:
hello
```

This is then passed to the command processor or HTTP endpoint.

### Environment Variables

The hooks system requires the following environment variables:

| Variable | Required | Description |
|----------|----------|-------------|
| `DISCORD_TOKEN` | Yes | Discord bot token with message content intent |
| `DISCORD_CHANNEL_ID` | No | Default channel for send/image commands |
| `HOOK_ENABLED` | No | Enable hook system (set to "true" or "1") |
| `HOOKS_FILE` | No | Path to hooks.yaml (default: `./hooks.yaml`) |
| `PROMPTS_DIR` | No | Path to prompts directory (default: `./prompts`) |
| `LOG_LEVEL` | No | Logging level: debug, info, warn, error |

#### Example .env File

```env
# Discord Bot Configuration
DISCORD_TOKEN=your_actual_discord_bot_token_here

# Hook System Configuration
HOOKS_FILE=./hooks.yaml
PROMPTS_DIR=./prompts
LOG_LEVEL=info
```

### Example Configurations

#### Simple Echo Command

Responds with the same message content:

```yaml
version: "1.0"

prompts_dir: "./prompts"

hooks:
  - id: "echo"
    name: "Echo Command"
    enabled: true
    channels:
      - "123456789012345678"
    trigger:
      type: "prefix"
      prefix: "!echo"
    prompt_file: "simple-echo.txt"
    action:
      type: "reply"
    processing:
      processor_type: "command"
      cmd: ["python", "-c", "import sys; print(sys.stdin.read())"]
```

**Prompt (`prompts/simple-echo.txt`):**
```
{{content}}
```

#### HTTP Webhook Integration

Forward matched messages to an external API:

```yaml
version: "1.0"

hooks:
  - id: "webhook-handler"
    name: "Webhook Handler"
    enabled: true
    channels:
      - "123456789012345678"
    trigger:
      type: "prefix"
      prefix: "!api"
    prompt_file: "api-request.txt"
    action:
      type: "reply"
    processing:
      processor_type: "http"
      url: "https://api.example.com/discord handler"
      timeout_seconds: 30
```

#### Regex Pattern Matching

Match messages using regex and respond accordingly:

```yaml
version: "1.0"

hooks:
  - id: "help-detector"
    name: "Help Request Detector"
    enabled: true
    channels:
      - "123456789012345678"
    trigger:
      type: "regex"
      pattern: "(?i)(help|support|need assistance)"
    prompt_file: "support.txt"
    action:
      type: "reply"
    processing:
      processor_type: "command"
      cmd: ["python", "scripts/handle_support.py"]
```

#### Forward to Channel

Automatically forward messages to another channel:

```yaml
version: "1.0"

hooks:
  - id: "log-commands"
    name: "Command Logger"
    enabled: true
    channels:
      - "123456789012345678"
    trigger:
      type: "prefix"
      prefix: "!"
    action:
      type: "forward"
      channel_id: "987654321098765432"
```

#### Rate-Limited Command Handler

With custom rate limiting:

```yaml
version: "1.0"

settings:
  on_error: "log"
  rate_limit:
    per_user: 3
    per_channel: 10
    window_seconds: 60

hooks:
  - id: "rate-limited-echo"
    name: "Rate Limited Echo"
    enabled: true
    channels:
      - "123456789012345678"
    trigger:
      type: "prefix"
      prefix: "!echo"
    prompt_file: "simple-echo.txt"
    action:
      type: "reply"
    processing:
      processor_type: "command"
      cmd: ["echo", "Processed: "]
```

### Using the Listen Command

Start the hook listener:

```bash
# Using default configuration (hooks.yaml)
discli listen

# Using custom hooks file
discli listen --hooks-file custom-hooks.yaml

# Using custom prompts directory
discli listen --prompts-dir ./custom-prompts

# Verbose output
discli listen --verbose
```

#### Listen Command Options

| Option | Short | Description |
|--------|-------|-------------|
| `--hooks-file` | `-f` | Path to hooks.yaml file |
| `--prompts-dir` | `-p` | Path to prompts directory |
| `--verbose` | `-v` | Enable verbose output |

The listener will connect to Discord and start monitoring configured channels. Press `Ctrl+C` to stop.

---

## Usage

### Basic Usage

discli uses a subcommand-based CLI structure. The primary commands are:

- `discli send` - Send messages (text or text with images)
- `discli image` - Send images with optional captions
- `discli listen` - Start hook listener to respond to Discord messages

### Send Messages

#### Text-Only Message

```bash
discli send "Hello from the command line!"
```

#### Message with Single Image

```bash
discli send "Check out this screenshot" --attach screenshot.png
```

#### Message with Multiple Images

```bash
discli send "Report attached" --attach fig1.png --attach fig2.png --attach fig3.png
```

#### Images Only (No Text)

```bash
discli send --attach photo.jpg
```

#### Message with Caption

```bash
discli send "Build complete" --attach result.png --caption "Deployment result"
```

### Using the Image Command

The `image` command is a convenience alias for sending images:

```bash
discli image --attach screenshot.jpg --caption "Error screenshot"
```

### Basic Examples

Send a simple message:

```bash
discli send "Hello from the command line!"
```

Send a message with emojis:

```bash
discli send "✅ Build completed successfully"
discli send "❌ Build failed"
discli send "⚠️ Warning: High CPU usage detected"
```

### Legacy Syntax (Deprecated)

For backward compatibility, the old syntax still works but shows a deprecation warning:

```bash
# Old way (deprecated)
discli "Hello, Discord!"

# New recommended way
discli send "Hello, Discord!"
```

### Advanced Examples

#### Multi-line Messages

Use newline characters (`\n`) for multi-line messages:

```bash
discli "Deployment Summary
Environment: Production
Status: Success
Duration: 2m 34s
Timestamp: 2024-02-15 14:30:00"
```

#### Using Shell Variables

Incorporate dynamic content using shell variables:

```bash
PROJECT_NAME="my-awesome-app"
BUILD_NUMBER="42"
discli "🎉 ${PROJECT_NAME} build #${BUILD_NUMBER} completed successfully"
```

#### Including Command Output

Capture and send command output:

```bash
discli "Disk usage: $(df -h / | tail -1)"
discli "Current time: $(date)"
```

#### Formatting Messages

Use consistent formatting for better readability:

```bash
discli "📊 System Report
━━━━━━━━━━━━━━━━━━
CPU: 45%
Memory: 62%
Disk: 78%
Uptime: 15 days
━━━━━━━━━━━━━━━━━━
Generated at: $(date)"
```

### CI/CD Integration

#### GitHub Actions

```yaml
name: Build and Notify
on:
  push:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build project
        run: cargo build --release

      - name: Setup discli
        run: |
          cargo install --path .
          echo "DISCORD_TOKEN=${{ secrets.DISCORD_TOKEN }}" > discli.env
          echo "DISCORD_CHANNEL_ID=${{ secrets.DISCORD_CHANNEL_ID }}" >> discli.env

      - name: Send notification on success
        if: success()
        run: discli send "✅ Build successful - ${{ github.event.head_commit.message }}"

      - name: Send notification on failure
        if: failure()
        run: discli send "❌ Build failed - ${{ github.event.head_commit.message }}"
```

#### GitLab CI

```yaml
build:
  stage: build
  script:
    - cargo build --release
    - cargo install --path .
    - echo "DISCORD_TOKEN=$DISCORD_TOKEN" > discli.env
    - echo "DISCORD_CHANNEL_ID=$DISCORD_CHANNEL_ID" >> discli.env
    - discli send "🚀 Pipeline $CI_PIPELINE_ID completed on $CI_COMMIT_REF_NAME"
  after_script:
    - |
      if [ $CI_JOB_STATUS == "success" ]; then
        discli send "✅ Build $CI_JOB_ID succeeded"
      else
        discli send "❌ Build $CI_JOB_ID failed"
      fi
```

#### Jenkins Pipeline

```groovy
pipeline {
    agent any
    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
        }
        stage('Notify') {
            steps {
                sh '''
                    cargo install --path .
                    echo "DISCORD_TOKEN=$DISCORD_TOKEN" > discli.env
                    echo "DISCORD_CHANNEL_ID=$DISCORD_CHANNEL_ID" >> discli.env
                    discli send "Build ${BUILD_NUMBER} completed: ${currentBuild.currentResult}"
                '''
            }
        }
    }
}
```

### Monitoring & Alerts

#### System Health Checks

```bash
#!/bin/bash
# health_check.sh

# Check if service is running
if curl -sf http://localhost:3000/health > /dev/null; then
    discli send "✅ Service healthy at $(date)"
else
    discli send "❌ Service down at $(date)"
fi
```

#### Disk Space Monitoring

```bash
#!/bin/bash
# disk_alert.sh

THRESHOLD=90
USAGE=$(df / | tail -1 | awk '{print $5}' | sed 's/%//')

if [ $USAGE -gt $THRESHOLD ]; then
    discli send "⚠️ Disk usage alert: ${USAGE}% used on root partition"
fi
```

#### Backup Monitoring

```bash
#!/bin/bash
# backup_notify.sh

if ./backup.sh; then
    discli send "✅ Backup completed successfully at $(date)"
else
    discli send "❌ Backup failed at $(date)"
fi
```

#### Alert with Screenshot

```bash
#!/bin/bash
# alert_with_screenshot.sh

# Take screenshot
scrot -u alert_screenshot.png

# Send alert with image
discli send "⚠️ High CPU detected" --attach alert_screenshot.png --caption "CPU usage graph"
```

#### Scheduled Cron Jobs

```bash
# Add to crontab for daily health check
0 8 * * * /path/to/health_check.sh
```

## API Reference

### Command Syntax

```bash
discli <subcommand> [options]
```

### Subcommands

| Command | Description |
|---------|-------------|
| `send` | Send a message (text or text with images) |
| `image` | Send images with optional captions |
| `listen` | Start hook listener to respond to Discord messages |

### Send Command Options

| Option | Short | Type | Description |
|--------|--------|------|-------------|
| `content` | - | string | Message content to send (optional) |
| `--attach` | `-a` | PATH | Image file(s) to attach (can be repeated) |
| `--caption` | `-c` | TEXT | Alt text/description for attachments |
| `--embed-url` | - | URL | Embed image URLs (future feature) |

### Image Command Options

| Option | Short | Type | Description |
|--------|--------|------|-------------|
| `--attach` | `-a` | PATH | Image file(s) to attach (required, can be repeated) |
| `--caption` | `-c` | TEXT | Caption text for the images |
| `--embed-url` | - | URL | Embed image URLs (future feature) |

### Listen Command Options

| Option | Short | Type | Description |
|--------|--------|------|-------------|
| `--hooks-file` | `-f` | PATH | Path to hooks.yaml file (default: `./hooks.yaml`) |
| `--prompts-dir` | `-p` | PATH | Path to prompts directory (default: `./prompts`) |
| `--verbose` | `-v` | flag | Enable verbose output |

### Environment Variables

| Variable | Required | Description |
|----------|-----------|-------------|
| `DISCORD_TOKEN` | Yes | Discord bot token |
| `DISCORD_CHANNEL_ID` | Yes | Discord channel ID to send messages to |

### Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Message sent successfully |
| `1` | Error occurred (missing args, config, or API error) |

### Standard Output

**Success:**
```
Successfully sent text message to channel 123456789012345678
```

**Success with Images:**
```
Successfully sent message with 2 image attachment(s) to channel 123456789012345678
```

### Standard Error

**Missing Command:**
```
Error: No subcommand provided
Usage: discli <subcommand> [options]
```

**Missing Configuration:**
```
Error: DISCORD_TOKEN environment variable not set
Please set it in your environment or in a discli.env file
```

```
Error: DISCORD_CHANNEL_ID environment variable not set
Please set it in your environment or in a discli.env file
```

**API Error:**
```
Error: Discord API error: Discord API returned error status 403: Missing Access
```

**Attachment Error:**
```
Error: Attachment error: File not found: /path/to/image.png
```

```
Error: Attachment error: File exceeds Discord's 25MB limit
```

**Validation Error:**
```
Error: Validation error: Cannot attach more than 10 images (got 11)
```

### Message Limitations

- Maximum message length: 2000 characters (Discord limit)
- Maximum attachments per message: 10 files
- Maximum file size: 25 MB per file
- Supported image formats: PNG, JPG, GIF, WebP, and other standard image formats
- Encoding: UTF-8

## Dependencies

`discli` is built with the following Rust dependencies:

| Crate | Version | Description |
|-------|---------|-------------|
| `reqwest` | 0.12 | HTTP client for making API requests (with multipart support) |
| `tokio` | 1.40 | Async runtime for Rust |
| `serde` | 1.0 | Serialization framework |
| `serde_json` | 1.0 | JSON serialization support |
| `dotenv` | 0.15 | Environment variable loading from `.env` files |
| `clap` | 4.5 | Command-line argument parsing |
| `mime_guess` | 2.0 | MIME type detection for file uploads |
| `thiserror` | 1.0 | Error handling library |

### System Requirements

- Operating System: Linux, macOS, or Windows
- Architecture: x86_64, ARM64, or others supported by Rust
- Network: Internet connection for Discord API access

## Troubleshooting

### Common Issues

#### Binary Not Found

**Problem:** `discli: command not found`

**Solution:**
- Ensure you've compiled the binary: `cargo build --release`
- Install globally: `cargo install --path .`
- Check your PATH includes the Cargo bin directory

#### Permission Denied

**Problem:** `Permission denied when running discli`

**Solution:**
- Make the binary executable: `chmod +x target/release/discli`
- Check file permissions on `discli.env`

#### Missing Environment Variables

**Problem:** `Error: DISCORD_TOKEN environment variable not set`

**Solution:**
- Verify `discli.env` exists and contains both `DISCORD_TOKEN` and `DISCORD_CHANNEL_ID`
- Check that `discli.env` is in the current directory when running `discli`
- Ensure there are no syntax errors in `discli.env` (no spaces around `=`)

#### Message Not Appearing

**Problem:** Command succeeds but message doesn't appear in Discord

**Solution:**
- Verify bot has `SEND_MESSAGES` permission in the channel
- Confirm `DISCORD_CHANNEL_ID` is correct
- Check bot token is valid and not revoked
- Ensure bot is actually added to the server

#### Rate Limiting Errors

**Problem:** `Discord API returned error status 429: You are being rate limited`

**Solution:**
- Discord allows bot messages at different rates per bot tier
- Implement delays between messages (minimum 1 second recommended)
- For high-volume notifications, consider a message queue

#### Network Errors

**Problem:** Connection timeout or network-related errors

**Solution:**
- Check internet connection
- Verify firewall allows connections to `discord.com`
- Check DNS resolution for `discord.com`

### Debug Mode

To troubleshoot issues, you can enable more verbose output:

```bash
# Run with shell debugging
bash -x discli "Test message"
```

### Getting Help

If you encounter issues not covered here:
- Check the [Discord API Documentation](https://discord.com/developers/docs)
- Review the bot permissions in Discord Developer Portal
- Verify your token hasn't been invalidated

## Development

### Building from Source

```bash
# Development build with debug symbols
cargo build

# Release build with optimizations
cargo build --release

# Run tests
cargo test

# Run with clippy for linting
cargo clippy
```

### Project Structure

```
discord-notifications/
├── Cargo.toml           # Project configuration and dependencies
├── Cargo.lock           # Dependency lock file
├── discli.env.example   # Example environment configuration
├── src/
│   └── main.rs          # Main application code
└── README.md            # This file
```

### Key Implementation Details

- Uses Discord REST API v10 endpoint: `POST /channels/{channel_id}/messages`
- Async operations via `tokio` runtime
- HTTP client via `reqwest` library
- Environment variable management via `dotenv`
- Proper error handling and user-friendly error messages

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Powered by [Discord API](https://discord.com/developers/docs)
- Inspired by the need for simple CI/CD notifications
