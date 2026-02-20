# Discord Notifications Tool (discli)

## Overview

This skill enables AI agents to send Discord notifications programmatically using the `discli` CLI tool. The tool provides a modern subcommand-based interface for sending text messages and image attachments to a configured Discord channel via a Discord bot.

### Key Capabilities

- **Text Messages**: Send plain text messages with rich formatting support
- **Image Attachments**: Upload images directly from local files (PNG, JPG, GIF, WebP, etc.)
- **Multiple Images**: Attach up to 10 images per message
- **Hybrid Messages**: Combine text content with image attachments
- **Flexible CLI**: Modern subcommand structure with intuitive flags
- **Backward Compatible**: Legacy syntax still works with deprecation warning

## Prerequisites

Before using this skill, ensure that:
- The `discli` binary is compiled and available in the system PATH
- A `discli.env` file exists in the project directory with the following configured:
  - `DISCORD_TOKEN`: Valid Discord bot token with message sending permissions
  - `DISCORD_CHANNEL_ID`: Target Discord channel ID

The agent assumes these are already configured and does not need to handle setup.

## Core Functionality

The tool uses a subcommand-based structure with two primary commands:

### Primary Commands

**Send Command**: Send messages (text or text with images)
```
discli send <content> [options]
```

**Image Command**: Send images with optional captions (convenience alias)
```
discli image --attach <file> [options]
```

### Legacy Syntax (Deprecated)

For backward compatibility, the old syntax still works:
```
discli <message>
```
*Note: This shows a deprecation warning and redirects to the new subcommand structure.*

### Input Structure

#### Send Command Parameters

**Mandatory Parameter:**
- `content` (string): The message content to send (optional if using `--attach`)

**Optional Flags:**
- `--attach`, `-a` (PATH): Image file(s) to attach (can be repeated, max 10)
- `--caption`, `-c` (TEXT): Alt text/description for attachments
- `--embed-url` (URL): Embed externally hosted image URLs (future feature)

**Constraints:**
- Message content cannot exceed Discord's 2000 character limit
- Max 10 total attachments per message (files + URLs combined)
- Max 25 MB per file (Discord's limit)
- Supported image formats: PNG, JPG, GIF, WebP, and other Discord-supported formats
- If no content provided, at least one `--attach` is required

#### Image Command Parameters

**Mandatory Parameters:**
- `--attach`, `-a` (PATH): At least one image file to attach (can be repeated, max 10)

**Optional Flags:**
- `--caption`, `-c` (TEXT): Caption text for the images (becomes message content)
- `--embed-url` (URL): Embed externally hosted image URLs (future feature)

**Constraints:**
- At least one `--attach` parameter is required
- Same limits as send command for file size and count

### Output Structure

**Success Output:**
- Returns exit code 0
- Prints to stdout:
  - Text-only: `Successfully sent text message to channel {channel_id}`
  - With images: `Successfully sent message with {N} image attachment(s) to channel {channel_id}`

**Error Conditions:**
- Returns exit code 1
- Prints error details to stderr with categorized error types:
  - Configuration errors (missing environment variables)
  - Validation errors (file not found, too many attachments)
  - Attachment errors (file size limit, invalid format)
  - API errors (network issues, Discord API responses)

## Usage Patterns

### Basic Usage

#### Text-Only Messages

Send a simple text message:
```bash
discli send "Hello, Discord!"
```

#### Messages with Images

Send a message with a single image:
```bash
discli send "Check out this screenshot" --attach screenshot.png
```

Send a message with multiple images:
```bash
discli send "Report attached" --attach fig1.png --attach fig2.png --attach fig3.png
```

Send images only (no text):
```bash
discli send --attach photo.jpg
```

Send a message with caption/description:
```bash
discli send "Build complete" --attach result.png --caption "Deployment result"
```

#### Using the Image Command

The `image` command is a convenience alias for sending images:
```bash
discli image --attach screenshot.jpg --caption "Error screenshot"
```

Send multiple images with a caption:
```bash
discli image --attach img1.png --attach img2.jpg -c "Analysis results"
```

### With Variables

When using dynamic content:
```bash
discli send "Build completed successfully for project ${PROJECT_NAME}"
```

With image attachments:
```bash
discli send "Build ${BUILD_NUMBER} results" --attach result_${BUILD_NUMBER}.png
```

### Multi-line Messages

For messages requiring line breaks:
```bash
discli send "Deployment status: SUCCESS

Time: $(date)
Environment: Production"
```

### Status Updates

Sending build or deployment notifications:
```bash
discli send "⚙️ Build #${BUILD_NUMBER} - SUCCESS
📦 Version: ${VERSION}
🌍 Environment: ${ENVIRONMENT}"
```

With deployment screenshots:
```bash
discli send "⚙️ Build #${BUILD_NUMBER} - SUCCESS
📦 Version: ${VERSION}
🌍 Environment: ${ENVIRONMENT}" --attach deployment_preview.png
```

## Integration Examples

### Shell Script Integration

```bash
#!/bin/bash
# Example: Deployment notification script

DEPLOY_START=$(date)
# ... deployment logic ...

if [ $? -eq 0 ]; then
    discli send "✅ Deployment completed successfully
Started: ${DEPLOY_START}
Finished: $(date)"
else
    discli send "❌ Deployment FAILED
Started: ${DEPLOY_START}
Finished: $(date)" --attach error_log.png
fi
```

### Deployment with Screenshots

```bash
#!/bin/bash
# Example: Deployment with screenshot notification

# Take screenshot after deployment
scrot deployment_result.png

# Send notification with screenshot
discli send "✅ Deployment completed successfully
Environment: ${ENVIRONMENT}
Build: ${BUILD_NUMBER}" --attach deployment_result.png --caption "Deployment result preview"
```

### CI/CD Pipeline Integration

```bash
# After a build step in CI/CD
if [ "$CI_JOB_STATUS" = "success" ]; then
    discli send "Pipeline #${CI_PIPELINE_ID} succeeded on ${CI_COMMIT_REF_NAME}"
else
    discli send "Pipeline #${CI_PIPELINE_ID} failed on ${CI_COMMIT_REF_NAME}" --attach build_log.png
fi
```

### Test Results with Screenshots

```bash
#!/bin/bash
# Run tests and capture screenshots on failure

if npm test; then
    discli send "✅ All tests passed
Commit: ${CI_COMMIT_SHA}
Branch: ${CI_COMMIT_REF_NAME}"
else
    # Capture screenshot of test failure
    scrot test_failure.png
    discli send "❌ Tests failed
Commit: ${CI_COMMIT_SHA}
Branch: ${CI_COMMIT_REF_NAME}" --attach test_failure.png --caption "Test failure screenshot"
fi
```

### Automated Monitoring

```bash
# Health check monitoring
if ! curl -sf http://localhost:3000/health > /dev/null; then
    discli send "⚠️ Service health check failed at $(date)"
fi
```

### Monitoring with Screenshots

```bash
#!/bin/bash
# Monitoring with visual alerts

THRESHOLD=90
CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | awk '{print 100 - $1}')

if [ $(echo "$CPU_USAGE > $THRESHOLD" | bc -l) -eq 1 ]; then
    # Take screenshot of system stats
    scrot cpu_alert.png
    discli send "⚠️ High CPU usage detected: ${CPU_USAGE}%" --attach cpu_alert.png --caption "CPU usage graph at $(date)"
fi
```

## Error Handling

The agent should be aware of these potential error conditions:

### Missing Subcommand
```
Error: No subcommand provided
Usage: discli <subcommand> [options]
```
**Action:** Use `discli send` or `discli image` subcommands

### Missing Arguments
```
Error: the required argument '<content>' was not provided
```
**Action:** Provide message content or use `--attach` flag with the send command

### Image Command Requirements
```
Error: the following required arguments were not provided:
  --attach <PATH>...
```
**Action:** The `image` command requires at least one `--attach` flag

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

### File Not Found
```
Error: File not found: /path/to/image.png
```
**Action:** Verify the file path is correct and the file exists

### File Size Limit Exceeded
```
Error: File exceeds Discord's 25MB limit
```
**Action:** Compress the image or reduce its size before attaching

### Too Many Attachments
```
Error: Cannot attach more than 10 images (got 11)
```
**Action:** Reduce the number of attachments to 10 or fewer

### Content Length Limit
```
Error: Message content exceeds Discord's 2000 character limit
```
**Action:** Shorten the message content

### Network/API Errors
```
Error: Discord API error: Discord API returned error status 403: Missing Access
```
```
Error: Discord API error: Discord API returned error status 404: Unknown Channel
```
**Action:** The bot token may lack permissions or channel ID is invalid. Verify configuration.

### Rate Limiting
```
Error: Discord API error: Discord API returned error status 429: You are being rate limited
```
**Action:** Implement retry logic with exponential backoff if sending multiple messages rapidly

### MIME Type Detection Error
```
Error: Unable to determine MIME type for file: image.unknown
```
**Action:** Ensure the file has a valid image extension or is in a supported format

## Best Practices

1. **Keep messages concise**: Discord has a 2000 character limit per message
2. **Use clear status indicators**: Include emojis or clear status prefixes (✅, ❌, ⚠️)
3. **Include timestamps**: Always include time information in automated messages
4. **Escape special characters**: If your message contains shell special characters, use single quotes or proper escaping
5. **Handle errors gracefully**: Always check the exit code and handle failures appropriately
6. **Provide captions**: Use the `--caption` flag to describe images for accessibility and clarity
7. **Optimize images**: Compress images before attaching to stay within the 25MB limit
8. **Verify file paths**: Always ensure image files exist before attempting to send
9. **Use appropriate commands**: Use `send` for text messages with optional images, use `image` when images are the primary content
10. **Monitor attachment count**: Be mindful of the 10 attachment limit per message

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

**Success with Screenshot:**
```
✅ Task completed successfully
Duration: 45s
Started: 2024-02-15 14:30:00

[Attachment: screenshot.png - "Result preview"]
```

**Error:**
```
❌ Operation failed
Error: Connection timeout
Attempted: 3 times
Timestamp: 2024-02-15 14:35:00
```

**Error with Logs:**
```
❌ Operation failed
Error: Connection timeout
Attempted: 3 times
Timestamp: 2024-02-15 14:35:00

[Attachment: error_log.png - "Error log screenshot"]
```

**Warning:**
```
⚠️ Resource usage high
CPU: 95%
Memory: 87%
Threshold: 80%
```

**Warning with Graph:**
```
⚠️ Resource usage high
CPU: 95%
Memory: 87%
Threshold: 80%

[Attachment: cpu_graph.png - "CPU usage over time"]
```

## Limitations

- Single message per invocation (no batch sending)
- Max 10 image attachments per message (Discord API limit)
- Max 25 MB per file attachment (Discord API limit)
- No message editing or deletion capabilities
- Synchronous operation (blocks until message sent or error occurs)
- No rich embeds with custom styling (basic image support only)
- Image URL embedding (`--embed-url`) is planned for future releases
- No support for other media types (videos, audio, documents)

## Troubleshooting

### Message Not Appearing in Channel
- Verify bot has `SEND_MESSAGES` permission in target channel
- Confirm `DISCORD_CHANNEL_ID` is correct
- Check bot token hasn't been revoked

### Images Not Uploading
- Verify file path is correct and file exists
- Check file size is under 25 MB limit
- Ensure file is in a supported image format (PNG, JPG, GIF, WebP, etc.)
- Check bot has `ATTACH_FILES` permission in target channel

### Frequent Rate Limit Errors
- Implement delay between messages (minimum 1 second recommended)
- Consider a message queue for high-volume notifications
- Reduce the number of images per message if sending many rapidly

### Binary Not Found
- Ensure `cargo install --path .` has been run to compile and install
- Verify installation directory is in system PATH

### "File Not Found" Errors
- Verify the image file path is absolute or relative to the current working directory
- Check file permissions and ensure the file is readable
- Use `ls -la` to verify the file exists before attempting to send

### MIME Type Detection Errors
- Ensure the image file has a valid extension (.png, .jpg, .jpeg, .gif, .webp)
- Verify the file is not corrupted
- Try converting the image to a different format

## Quick Reference

| Action | Command |
|--------|---------|
| Send simple text message | `discli send "Hello"` |
| Send multi-line message | `discli send "Line 1\nLine 2"` |
| Include variables | `discli send "Status: ${STATUS}"` |
| Send emoji notification | `discli send "✅ Done"` |
| Send message with single image | `discli send "Check this" --attach img.png` |
| Send message with multiple images | `discli send "Report" -a img1.png -a img2.jpg` |
| Send images only (no text) | `discli send --attach photo.jpg` |
| Send image with caption | `discli image -a screenshot.jpg -c "Error"` |
| Send multiple images with caption | `discli image -a img1.png -a img2.jpg -c "Analysis"` |
| Legacy syntax (deprecated) | `discli "Hello"` |

## Technical Details

- **Binary name**: `discli`
- **Environment file**: `discli.env`
- **API endpoint**: Discord REST API v10
- **HTTP method**: POST to `/channels/{channel_id}/messages`
- **Authentication**: Bot token in Authorization header
- **Content-Type**: 
  - Text-only messages: `application/json`
  - Messages with attachments: `multipart/form-data`

### Module Architecture

The tool follows a modular architecture:

```
src/
├── main.rs           # Entry point and subcommand routing
├── cli.rs            # CLI argument definitions (clap)
├── commands/         # Command implementations
│   ├── mod.rs
│   ├── send.rs       # Send message command (text + images)
│   └── image.rs      # Image-specific command (convenience)
├── discord/          # Discord API layer
│   ├── mod.rs
│   ├── api.rs        # API client and request handling
│   ├── client.rs     # Discord HTTP client
│   └── types.rs      # Discord API types
├── message/          # Message construction
│   ├── mod.rs
│   ├── builder.rs    # Message builder pattern
│   ├── attachment.rs # File attachment handling
│   └── validation.rs # Input validation
├── config/           # Configuration management
│   ├── mod.rs
│   └── env.rs        # Environment variable loading
└── error.rs          # Error types and handling
```

### Key Dependencies

- `clap` ~4.5: CLI argument parsing with subcommand support
- `reqwest` ~0.12: HTTP client for Discord API requests
- `tokio` ~1.40: Async runtime
- `serde` ~1.0: JSON serialization
- `dotenv` ~0.15: Environment variable loading
- `mime_guess` ~0.4: MIME type detection for attachments

## Image Functionality

### Supported Image Formats

Discli supports all Discord-compatible image formats, including:
- PNG (.png)
- JPEG/JPG (.jpg, .jpeg)
- GIF (.gif)
- WebP (.webp)
- And other formats supported by Discord

### Image Upload Workflow

When attaching images, discli follows this workflow:

1. **Validation**: Check file exists and is within size limits (25 MB max)
2. **MIME Detection**: Automatically determine the file's MIME type
3. **Multipart Construction**: Build multipart/form-data request with:
   - Content field (optional text message)
   - File attachments (binary data with metadata)
4. **API Upload**: Send to Discord's `/channels/{id}/messages` endpoint
5. **Response Handling**: Confirm successful upload or report errors

### Multiple Images

Send multiple images in a single message:

```bash
# Using send command
discli send "Analysis complete" --attach chart1.png --attach chart2.jpg --attach chart3.png

# Using image command
discli image -a chart1.png -a chart2.jpg -a chart3.png -c "Analysis results"
```

### Image Captions

Use the `--caption` flag to provide alt text or descriptions:

```bash
discli send "Deployment snapshot" --attach screenshot.png --caption "Successful deployment of version 2.1.0"
```

Captions serve multiple purposes:
- Accessibility for screen readers
- Context for what the image shows
- Documentation in Discord message history

### File Size Considerations

Discord has strict size limits:
- **25 MB per file** for bots
- **8 MB per file** for non-Nitro users in DMs

Best practices:
- Compress images before sending if they exceed limits
- Use appropriate image formats (WebP is often smaller than PNG)
- Consider splitting large datasets into multiple images

### Best Practices for Images

1. **Optimize file size**: Compress images before sending
2. **Provide captions**: Always use `--caption` for accessibility and clarity
3. **Use appropriate formats**: Choose formats that balance quality and size
4. **Verify file paths**: Ensure files exist before sending
5. **Monitor attachment count**: Stay within the 10 attachment limit

## Version Compatibility

This skill is designed for `discli` version 0.2.0 and later. This version introduced:

- Subcommand-based architecture (`send` and `image` commands)
- Image attachment support with multipart/form-data uploads
- Multiple images per message (up to 10)
- MIME type detection and file validation
- Comprehensive error handling and validation
- Backward compatibility with legacy syntax (deprecated)

### Migration from Previous Versions

If migrating from discli 0.1.0:

1. **Update command syntax**:
   - Old: `discli "Hello"` → New: `discli send "Hello"`
   
2. **Add subcommand** to all existing invocations

3. **Image support** is now available (new capability in 0.2.0)

4. **Legacy syntax still works** but shows a deprecation warning

Future versions may introduce additional features such as:
- Image URL embedding (`--embed-url`)
- Rich embeds with custom styling
- Message editing capabilities
- Additional media types
