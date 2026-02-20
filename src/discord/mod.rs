//! Discord API module
//!
//! This module provides functionality for interacting with Discord API,
//! including sending messages with attachments.

pub mod api;
pub mod client;
pub mod gateway;
pub mod types;

pub use client::DiscordClient;
pub use gateway::{create_gateway, DiscordGateway};

