# Discord Notifications Tool (discli)

## Overview

This skill enables AI agents to send Discord notifications programmatically using the `discli` CLI tool. The tool provides a simple command-line interface for sending messages to a configured Discord channel via a Discord bot.

## Prerequisites

Before using this skill, ensure that:
- The `discli` binary is compiled and available in the system PATH
- A `discli.env` file exists in the project directory with the following configured:
  - `DISCORD_TOKEN`: Valid Discord bot token with message sending permissions
  - `DISCORD_CHANNEL_ID`: Target Discord channel ID

The agent assumes these are already configured and does not need to handle setup.

## Core Functionality

The tool sends messages to Discord via a single command:
```
discli <message>
```

### Input Structure

**Mandatory Parameter:**
- `message` (string): The content to send to the Discord channel

**Constraints:**
- Message cannot be empty
- Message length must not exceed Discord's 2000 character limit
- Message is sent as plain text (no formatting or embedded support in current version)

### Output Structure

**Success Output:**
- Returns exit code 0
- Prints to stdout: `Message sent successfully to channel {channel_id}`

**Error Conditions:**
- Returns exit code 1
- Prints error details to stderr

## Usage Patterns

### Basic Usage

Send a simple text message:
```bash
discli "Hello, Discord!"
```

### With Variables

When using dynamic content:
```bash
discli "Build completed successfully for project ${PROJECT_NAME}"
```

### Multi-line Messages

For messages requiring line breaks:
```bash
discli "Deployment status: SUCCESS

Time: $(date)
Environment: Production"
```

### Status Updates

Sending build or deployment notifications:
```bash
discli "⚙️ Build #${BUILD_NUMBER} - SUCCESS
📦 Version: ${VERSION}
🌍 Environment: ${ENVIRONMENT}"
```

## Integration Examples

### Shell Script Integration

```bash
#!/bin/bash
# Example: Deployment notification script

DEPLOY_START=$(date)
# ... deployment logic ...

if [ $? -eq 0 ]; then
    discli "✅ Deployment completed successfully
Started: ${DEPLOY_START}
Finished: $(date)"
else
    discli "❌ Deployment FAILED
Started: ${DEPLOY_START}
Finished: $(date)"
fi
```

### CI/CD Pipeline Integration

```bash
# After a build step in CI/CD
if [ "$CI_JOB_STATUS" = "success" ]; then
    discli "Pipeline #${CI_PIPELINE_ID} succeeded on ${CI_COMMIT_REF_NAME}"
else
    discli "Pipeline #${CI_PIPELINE_ID} failed on ${CI_COMMIT_REF_NAME}"
fi
```

### Automated Monitoring

```bash
# Health check monitoring
if ! curl -sf http://localhost:3000/health > /dev/null; then
    discli "⚠️ Service health check failed at $(date)"
fi
```

## Error Handling

The agent should be aware of these potential error conditions:

### Missing Arguments
```
Error: No message provided
Usage: discli <message>
```
**Action:** Ensure a message argument is provided

### Missing Environment Variables
```
Error: DISCORD_CHANNEL_ID environment variable not set
Please set it in your environment or in a discli.env file
```
```
Error: DISCORD_TOKEN environment variable not set
Please set it in your environment or in a discli.env file
```
**Action:** Verify `discli.env` exists with required variables

### Network/API Errors
```
Error sending message: Discord API returned error status 403: Missing Access
```
```
Error sending message: Discord API returned error status 404: Unknown Channel
```
**Action:** The bot token may lack permissions or channel ID is invalid. Verify configuration.

### Rate Limiting
```
Error sending message: Discord API returned error status 429: You are being rate limited
```
**Action:** Implement retry logic with exponential backoff if sending multiple messages rapidly

## Best Practices

1. **Keep messages concise**: Discord has a 2000 character limit per message
2. **Use clear status indicators**: Include emojis or clear status prefixes (✅, ❌, ⚠️)
3. **Include timestamps**: Always include time information in automated messages
4. **Escape special characters**: If your message contains shell special characters, use single quotes or proper escaping
5. **Handle errors gracefully**: Always check the exit code and handle failures appropriately

## Message Formatting Guidelines

### Recommended Message Structure
```
[Status Emoji] Brief status summary
Details line 1
Details line 2
Timestamp: [time]
```

### Example Templates

**Success:**
```
✅ Task completed successfully
Duration: 45s
Started: 2024-02-15 14:30:00
```

**Error:**
```
❌ Operation failed
Error: Connection timeout
Attempted: 3 times
Timestamp: 2024-02-15 14:35:00
```

**Warning:**
```
⚠️ Resource usage high
CPU: 95%
Memory: 87%
Threshold: 80%
```

## Limitations

- Single message per invocation (no batch sending)
- Plain text only (no rich embeds or attachments in current version)
- No message editing or deletion capabilities
- Synchronous operation (blocks until message sent or error occurs)

## Troubleshooting

### Message Not Appearing in Channel
- Verify bot has `SEND_MESSAGES` permission in target channel
- Confirm `DISCORD_CHANNEL_ID` is correct
- Check bot token hasn't been revoked

### Frequent Rate Limit Errors
- Implement delay between messages (minimum 1 second recommended)
- Consider a message queue for high-volume notifications

### Binary Not Found
- Ensure `cargo install --path .` has been run to compile and install
- Verify installation directory is in system PATH

## Quick Reference

| Action | Command |
|--------|---------|
| Send simple message | `discli "Hello"` |
| Send multi-line message | `discli "Line 1\nLine 2"` |
| Include variables | `discli "Status: ${STATUS}"` |
| Send emoji notification | `discli "✅ Done"` |

## Technical Details

- **Binary name**: `discli`
- **Environment file**: `discli.env`
- **API endpoint**: Discord REST API v10
- **HTTP method**: POST to `/channels/{channel_id}/messages`
- **Authentication**: Bot token in Authorization header
- **Content-Type**: application/json

## Version Compatibility

This skill is designed for `discli` version 0.1.0. Future versions may introduce additional features or change behavior.
