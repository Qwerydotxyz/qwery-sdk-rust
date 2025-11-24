//! # Qwery SDK
//!
//! Rust SDK for integrating Qwery x402 Payment Facilitator into your applications.
//!
//! ## Features
//!
//! - Zero user fees - Facilitator pays network costs
//! - Instant settlement - Sub-2 second transaction finality
//! - Multi-token support - SOL, USDC, USDT on Solana
//! - Type-safe Rust API
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use qwery_sdk::{QweryClient, PaymentRequest, Network};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = QweryClient::new(Network::Mainnet)?;
//!
//!     let payment = client.create_payment(PaymentRequest {
//!         amount: 0.01,
//!         token: "SOL".to_string(),
//!         recipient: "recipient_wallet_address".to_string(),
//!         metadata: None,
//!     }).await?;
//!
//!     println!("Payment ID: {}", payment.payment_id);
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod types;
pub mod error;

pub use client::QweryClient;
pub use types::*;
pub use error::QweryError;
