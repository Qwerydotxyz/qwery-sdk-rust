//! Error types for Qwery SDK

use thiserror::Error;

/// Errors that can occur when using the Qwery SDK
#[derive(Error, Debug)]
pub enum QweryError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    /// JSON serialization/deserialization failed
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// API returned an error
    #[error("API error: {0}")]
    ApiError(String),

    /// Invalid configuration
    #[error("Invalid configuration: {0}")]
    ConfigError(String),

    /// Transaction signing failed
    #[error("Signing error: {0}")]
    SigningError(String),

    /// Base64 encoding/decoding failed
    #[error("Base64 error: {0}")]
    Base64Error(#[from] base64::DecodeError),

    /// Solana SDK error
    #[error("Solana error: {0}")]
    SolanaError(String),
}

/// Result type for Qwery SDK operations
pub type Result<T> = std::result::Result<T, QweryError>;
