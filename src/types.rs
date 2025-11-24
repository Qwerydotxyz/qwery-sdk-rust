//! Types used in the Qwery SDK

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Network to connect to
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Solana Mainnet
    Mainnet,
    /// Solana Devnet (for testing)
    Devnet,
}

impl Network {
    /// Get the network name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Network::Mainnet => "solana",
            Network::Devnet => "solana-devnet",
        }
    }
}

/// Configuration for the Qwery client
#[derive(Debug, Clone)]
pub struct QweryConfig {
    /// Facilitator API URL
    pub facilitator_url: String,
    /// Network to use
    pub network: Network,
    /// Optional API key
    pub api_key: Option<String>,
}

impl Default for QweryConfig {
    fn default() -> Self {
        Self {
            facilitator_url: "https://facilitator.qwery.xyz".to_string(),
            network: Network::Mainnet,
            api_key: None,
        }
    }
}

/// Request to create a payment
#[derive(Debug, Clone, Serialize)]
pub struct PaymentRequest {
    /// Amount to pay
    pub amount: f64,
    /// Token to use (SOL, USDC, USDT)
    pub token: String,
    /// Recipient wallet address
    pub recipient: String,
    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

/// Response from creating a payment
#[derive(Debug, Clone, Deserialize)]
pub struct PaymentResponse {
    /// Unique payment ID
    pub payment_id: String,
    /// Base64 encoded transaction to sign
    pub transaction: String,
    /// Amount in token
    pub amount: f64,
    /// Token used
    pub token: String,
    /// Recipient address
    pub recipient: String,
    /// Network used
    pub network: String,
    /// Payment status
    pub status: String,
    /// Expiration timestamp
    pub expires_at: Option<String>,
}

/// Request to settle a payment
#[derive(Debug, Clone, Serialize)]
pub struct SettleRequest {
    /// Payment ID to settle
    pub payment_id: String,
    /// Base64 encoded signed transaction
    pub signed_transaction: String,
}

/// Response from settling a payment
#[derive(Debug, Clone, Deserialize)]
pub struct SettleResponse {
    /// Whether settlement was successful
    pub success: bool,
    /// Transaction signature
    pub signature: Option<String>,
    /// Payment status
    pub status: String,
    /// Error message if failed
    pub error: Option<String>,
}

/// Request to verify a payment
#[derive(Debug, Clone, Serialize)]
pub struct VerifyRequest {
    /// Transaction signature to verify
    pub signature: String,
    /// Network to verify on
    pub network: String,
}

/// Response from verifying a payment
#[derive(Debug, Clone, Deserialize)]
pub struct VerifyResponse {
    /// Whether the payment is verified
    pub verified: bool,
    /// Payment status
    pub status: String,
    /// Confirmation count
    pub confirmations: Option<u64>,
}

/// Health status of the facilitator
#[derive(Debug, Clone, Deserialize)]
pub struct HealthResponse {
    /// Overall status
    pub status: String,
    /// API version
    pub version: String,
    /// Network statuses
    pub networks: HashMap<String, String>,
}
