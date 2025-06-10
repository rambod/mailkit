//! MailKit is a small library for sending emails.
//!
//! It exposes a simple logger, a JSON helper type and an [`EmailSender`]
//! which provides sync and async email sending. See the individual modules
//! for more details.

#![forbid(unsafe_code)]

pub mod email_sender;
pub mod simple_logger;
pub mod json;

pub use email_sender::{EmailSender, SendAgent};
pub use simple_logger::SimpleLogger;
pub use json::JsonValue;
