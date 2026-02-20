# discli - Discord Notifications CLI

A simple command-line tool for sending Discord notifications programmatically. `discli` enables you to send messages to any Discord channel via a bot with a single command, making it perfect for CI/CD pipelines, automated monitoring, and system alerts.

## Features

- 🚀 **Simple CLI Interface**: Send messages with a single command
- 📸 **Image Support**: Attach images to your Discord messages
- ⚙️ **Environment-based Configuration**: Secure token management via `.env` file
- 📡 **Async Operations**: Built on Tokio for efficient HTTP requests
- 🔒 **Secure**: No hardcoded credentials - all configuration via environment variables
- 🎯 **Lightweight**: Minimal dependencies and small binary size

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Configuration](#configuration)
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

## Usage

### Basic Usage

discli uses a subcommand-based CLI structure. The primary commands are:

- `discli send` - Send messages (text or text with images)
- `discli image` - Send images with optional captions

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
