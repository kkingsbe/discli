//! discli - Discord CLI with image support
//!
//! A command-line tool for sending Discord notifications with support for
//! text messages and image attachments.

mod cli;
mod commands;
mod config;
mod discord;
mod error;
mod hooks;
mod message;
mod processing;
mod prompt;

use clap::Parser;
use error::Result;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Main application entry point
async fn run() -> Result<()> {
    // Parse CLI arguments
    let cli = cli::Cli::parse();

    // Load configuration from environment
    let config = config::Config::load()?;

    // Handle backward compatibility for legacy syntax
    if !cli.legacy_message.is_empty() {
        handle_legacy_syntax(&config, cli.legacy_message).await
    } else {
        handle_subcommands(&config, cli.command).await
    }
}

/// Handle legacy syntax for backward compatibility
///
/// Legacy syntax: `discli "message"`
/// New syntax: `discli send "message"`
async fn handle_legacy_syntax(config: &config::Config, legacy_message: Vec<String>) -> Result<()> {
    // Warn about deprecation
    eprintln!("⚠️  Warning: Direct message argument is deprecated.");
    eprintln!("  Current: discli \"message\"");
    eprintln!("  New:     discli send \"message\"");
    eprintln!();

    // Use legacy behavior
    let content = legacy_message.join(" ");
    commands::send::execute(config, content, Vec::new(), Vec::new(), None).await
}

/// Handle subcommands
async fn handle_subcommands(
    config: &config::Config,
    command: Option<cli::Commands>,
) -> Result<()> {
    match command {
        Some(cli::Commands::Send {
            content,
            attach,
            embed_url,
            caption,
        }) => {
            commands::send::execute(config, content, attach, embed_url, caption).await
        }
        Some(cli::Commands::Image {
            attach,
            caption,
            embed_url,
        }) => {
            commands::image::execute(config, attach, caption, embed_url).await
        }
        Some(cli::Commands::Listen {
            foreground,
            hooks_file,
            prompts_dir,
            verbose,
        }) => {
            commands::listen::execute(config, hooks_file, prompts_dir, verbose).await
        }
        None => {
            // No subcommand provided, show help
            // Use clap's built-in help
            std::process::exit(0);
        }
    }
}
