//! Processing backends for hooks
//! 
//! Provides different ways to process messages:
//! - Command execution
//! - HTTP webhook calls

pub mod command;
pub mod http;

pub use command::CommandProcessor;
pub use http::HttpProcessor;
