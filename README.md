<div align="center">
  <img src="https://res.cloudinary.com/dkfwg4ta8/image/upload/v1763920160/sdkts_wocykm.png" alt="Qwery SDK Banner" width="100%" />
</div>

# Qwery SDK for Rust

Rust SDK for integrating Qwery x402 Payment Facilitator into your applications.

[![Crates.io](https://img.shields.io/crates/v/qwery-sdk.svg)](https://crates.io/crates/qwery-sdk)
[![Documentation](https://docs.rs/qwery-sdk/badge.svg)](https://docs.rs/qwery-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
qwery-sdk = "0.1"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start
```rust
use qwery_sdk::{QweryClient, PaymentRequest, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = QweryClient::new(Network::Mainnet)?;

    // Create payment
    let payment = client.create_payment(PaymentRequest {
        amount: 0.01,
        token: "SOL".to_string(),
        recipient: "merchant_wallet_address".to_string(),
        metadata: None,
    }).await?;

    println!("Payment ID: {}", payment.payment_id);
    println!("Transaction: {}", payment.transaction);

    Ok(())
}
```

## Features

- **Zero User Fees** - Facilitator pays network costs
- **Instant Settlement** - Sub-2 second transaction finality
- **Multi-Token Support** - SOL, USDC, USDT on Solana
- **Type-Safe** - Full Rust type safety
- **Async/Await** - Built on Tokio for async operations

## Examples

### Sign and Settle Payment
```rust
use qwery_sdk::{QweryClient, PaymentRequest, Network};
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QweryClient::new(Network::Devnet)?;
    let keypair = Keypair::new();

    // Create payment
    let payment = client.create_payment(PaymentRequest {
        amount: 0.01,
        token: "SOL".to_string(),
        recipient: "recipient_address".to_string(),
        metadata: None,
    }).await?;

    // Sign and settle
    let result = client.sign_and_settle(&payment, &keypair).await?;
    
    if result.success {
        println!("Payment settled! Signature: {:?}", result.signature);
    }

    Ok(())
}
```

### Verify Payment
```rust
use qwery_sdk::{QweryClient, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QweryClient::new(Network::Mainnet)?;

    let result = client.verify_payment("transaction_signature").await?;
    
    println!("Verified: {}", result.verified);
    println!("Status: {}", result.status);

    Ok(())
}
```

### Health Check
```rust
use qwery_sdk::{QweryClient, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QweryClient::new(Network::Mainnet)?;

    let health = client.health().await?;
    
    println!("Status: {}", health.status);
    println!("Version: {}", health.version);

    Ok(())
}
```

## Documentation

- **API Docs**: https://docs.rs/qwery-sdk
- **Facilitator API**: https://facilitator.qwery.xyz/docs
- **Website**: https://qwery.xyz

## License

MIT © Qwery
