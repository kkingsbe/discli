use reqwest::Client;
use serde_json::json;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    // Load environment variables from discli.env file if it exists
    dotenv::from_filename("discli.env").ok();

    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <message>", args[0]);
        eprintln!("Environment Variables:");
        eprintln!("  DISCORD_TOKEN - Your Discord bot token (required)");
        eprintln!("  DISCORD_CHANNEL_ID - Your Discord channel ID (required)");
        eprintln!("\nExample:");
        eprintln!("  {} \"Hello from Rust!\"", args[0]);
        process::exit(1);
    }

    let message = &args[1];

    // Get Discord channel ID from environment variable
    let channel_id = match env::var("DISCORD_CHANNEL_ID") {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Error: DISCORD_CHANNEL_ID environment variable not set");
            eprintln!("Please set it in your environment or in a discli.env file");
            process::exit(1);
        }
    };

    // Get Discord token from environment variable
    let token = match env::var("DISCORD_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!("Error: DISCORD_TOKEN environment variable not set");
            eprintln!("Please set it in your environment or in a discli.env file");
            process::exit(1);
        }
    };

    // Send the message
    match send_discord_message(&token, &channel_id, message).await {
        Ok(_) => {
            println!("Message sent successfully to channel {}", channel_id);
        }
        Err(e) => {
            eprintln!("Error sending message: {}", e);
            process::exit(1);
        }
    }
}

async fn send_discord_message(
    token: &str,
    channel_id: &str,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client
    let client = Client::new();

    // Construct the API URL
    let url = format!(
        "https://discord.com/api/v10/channels/{}/messages",
        channel_id
    );

    // Prepare the request body
    let body = json!({
        "content": message
    });

    // Send POST request to Discord API
    let response = client
        .post(&url)
        .header("Authorization", format!("Bot {}", token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    // Check if the request was successful
    let status = response.status();
    
    if !status.is_success() {
        let error_text = response.text().await?;
        return Err(format!(
            "Discord API returned error status {}: {}",
            status, error_text
        )
        .into());
    }

    Ok(())
}

