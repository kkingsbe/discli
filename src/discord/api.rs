//! Discord API request handlers

use crate::discord::types::FileAttachment;
use crate::error::{DiscliError, Result};
use reqwest::Client;
use serde_json::json;
use tokio::io::AsyncReadExt;

/// Send a simple JSON message to Discord
///
/// # Arguments
///
/// * `client` - HTTP client to use for the request
/// * `url` - Full API URL to send the message to
/// * `token` - Discord bot token
/// * `content` - Message content to send
///
/// # Returns
///
/// `Ok(())` if the message was sent successfully
///
/// # Errors
///
/// Returns an error if the HTTP request fails or Discord returns an error
pub async fn send_json_message(
    client: &Client,
    url: &str,
    token: &str,
    content: &str,
) -> Result<()> {
    let body = json!({
        "content": content
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    check_response(response).await
}

/// Send a multipart/form-data message with attachments to Discord
///
/// # Arguments
///
/// * `client` - HTTP client to use for the request
/// * `url` - Full API URL to send the message to
/// * `token` - Discord bot token
/// * `content` - Optional message content
/// * `attachments` - List of file attachments to include
///
/// # Returns
///
/// `Ok(())` if the message was sent successfully
///
/// # Errors
///
/// Returns an error if:
/// - Reading any attachment file fails
/// - Building the multipart form fails
/// - The HTTP request fails
/// - Discord returns an error
pub async fn send_multipart_message(
    client: &Client,
    url: &str,
    token: &str,
    content: &Option<String>,
    attachments: &[FileAttachment],
) -> Result<()> {
    let mut form = reqwest::multipart::Form::new();

    // Add payload_json if we have attachments (or content to send)
    if !attachments.is_empty() || content.is_some() {
        // Build attachments array
        let mut payload_attachments = Vec::new();
        for (index, attachment) in attachments.iter().enumerate() {
            let attachment_json = if let Some(desc) = &attachment.description {
                json!({
                    "id": index,
                    "description": desc,
                })
            } else {
                json!({
                    "id": index,
                })
            };
            payload_attachments.push(attachment_json);
        }

        // Build payload_json with content and attachments
        // Note: Discord requires content to be in payload_json when using multipart
        let payload_json = if let Some(text) = content {
            json!({
                "content": text,
                "attachments": payload_attachments
            })
        } else {
            json!({
                "attachments": payload_attachments
            })
        };
        form = form.text("payload_json", payload_json.to_string());
    } else if let Some(text) = content {
        // No attachments but have content - use simple content field
        form = form.text("content", text.clone());
    }

    // Add attachments
    for (index, attachment) in attachments.iter().enumerate() {
        let mut file = tokio::fs::File::open(&attachment.path).await?;
        let file_len = file.metadata().await?.len();

        // Validate size (Discord limit: 25MB)
        if file_len > 25 * 1024 * 1024 {
            return Err(DiscliError::Attachment(format!(
                "File too large: {} exceeds 25MB limit",
                attachment.path.display()
            )));
        }

        // Read file into bytes
        let mut buffer = Vec::with_capacity(file_len as usize);
        file.read_to_end(&mut buffer).await?;

        let part = reqwest::multipart::Part::bytes(buffer)
            .file_name(attachment.filename.clone())
            .mime_str(&attachment.mime_type)
            .map_err(|e| DiscliError::Mime(format!("Invalid MIME type: {}", e)))?;

        let key = format!("files[{}]", index);
        form = form.part(key, part);
    }

    // Send request
    let response = client
        .post(url)
        .header("Authorization", format!("Bot {}", token))
        .multipart(form)
        .send()
        .await?;

    check_response(response).await
}

/// Check HTTP response and handle errors
async fn check_response(response: reqwest::Response) -> Result<()> {
    let status = response.status();

    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(DiscliError::DiscordApi(format!(
            "Discord API returned error status {}: {}",
            status, error_text
        )));
    }

    Ok(())
}
