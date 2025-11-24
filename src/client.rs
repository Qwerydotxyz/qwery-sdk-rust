//! Qwery API client

use crate::error::{QweryError, Result};
use crate::types::*;
use reqwest::Client;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use solana_sdk::{
    signature::Keypair,
    transaction::Transaction,
};

/// Main client for interacting with the Qwery API
#[derive(Debug, Clone)]
pub struct QweryClient {
    config: QweryConfig,
    http_client: Client,
}

impl QweryClient {
    /// Create a new Qwery client with the specified network
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use qwery_sdk::{QweryClient, Network};
    ///
    /// let client = QweryClient::new(Network::Mainnet).unwrap();
    /// ```
    pub fn new(network: Network) -> Result<Self> {
        let config = QweryConfig {
            network,
            ..Default::default()
        };
        Self::with_config(config)
    }

    /// Create a new Qwery client with custom configuration
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use qwery_sdk::{QweryClient, QweryConfig, Network};
    ///
    /// let config = QweryConfig {
    ///     facilitator_url: "https://facilitator.qwery.xyz".to_string(),
    ///     network: Network::Devnet,
    ///     api_key: Some("your_api_key".to_string()),
    /// };
    /// let client = QweryClient::with_config(config).unwrap();
    /// ```
    pub fn with_config(config: QweryConfig) -> Result<Self> {
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(QweryError::RequestError)?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create a new payment request
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use qwery_sdk::{QweryClient, PaymentRequest, Network};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = QweryClient::new(Network::Mainnet)?;
    ///
    ///     let payment = client.create_payment(PaymentRequest {
    ///         amount: 0.01,
    ///         token: "SOL".to_string(),
    ///         recipient: "recipient_address".to_string(),
    ///         metadata: None,
    ///     }).await?;
    ///
    ///     println!("Payment ID: {}", payment.payment_id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_payment(&self, request: PaymentRequest) -> Result<PaymentResponse> {
        let url = format!("{}/payments/create", self.config.facilitator_url);

        let mut req = self.http_client.post(&url).json(&serde_json::json!({
            "amount": request.amount,
            "token": request.token,
            "recipient": request.recipient,
            "network": self.config.network.as_str(),
            "metadata": request.metadata,
        }));

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(QweryError::ApiError(error_text));
        }

        let payment: PaymentResponse = response.json().await?;
        Ok(payment)
    }

    /// Sign and settle a payment using a keypair
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use qwery_sdk::{QweryClient, PaymentRequest, Network};
    /// use solana_sdk::signature::Keypair;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = QweryClient::new(Network::Devnet)?;
    ///     let keypair = Keypair::new();
    ///
    ///     let payment = client.create_payment(PaymentRequest {
    ///         amount: 0.01,
    ///         token: "SOL".to_string(),
    ///         recipient: "recipient_address".to_string(),
    ///         metadata: None,
    ///     }).await?;
    ///
    ///     let result = client.sign_and_settle(&payment, &keypair).await?;
    ///     println!("Signature: {:?}", result.signature);
    ///     Ok(())
    /// }
    /// ```
    pub async fn sign_and_settle(
        &self,
        payment: &PaymentResponse,
        keypair: &Keypair,
    ) -> Result<SettleResponse> {
        // Decode the transaction
        let tx_bytes = BASE64.decode(&payment.transaction)
            .map_err(QweryError::Base64Error)?;

        let mut transaction: Transaction = bincode::deserialize(&tx_bytes)
            .map_err(|e| QweryError::SolanaError(e.to_string()))?;

        // Sign the transaction
        transaction.partial_sign(&[keypair], transaction.message.recent_blockhash);

        // Encode the signed transaction
        let signed_bytes = bincode::serialize(&transaction)
            .map_err(|e| QweryError::SolanaError(e.to_string()))?;
        let signed_base64 = BASE64.encode(&signed_bytes);

        // Settle the payment
        self.settle_payment(SettleRequest {
            payment_id: payment.payment_id.clone(),
            signed_transaction: signed_base64,
        }).await
    }

    /// Settle a payment with a pre-signed transaction
    pub async fn settle_payment(&self, request: SettleRequest) -> Result<SettleResponse> {
        let url = format!("{}/payments/settle", self.config.facilitator_url);

        let mut req = self.http_client.post(&url).json(&request);

        if let Some(ref api_key) = self.config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(QweryError::ApiError(error_text));
        }

        let settle_response: SettleResponse = response.json().await?;
        Ok(settle_response)
    }

    /// Verify a payment by transaction signature
    pub async fn verify_payment(&self, signature: &str) -> Result<VerifyResponse> {
        let url = format!("{}/payments/verify", self.config.facilitator_url);

        let response = self.http_client
            .post(&url)
            .json(&VerifyRequest {
                signature: signature.to_string(),
                network: self.config.network.as_str().to_string(),
            })
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(QweryError::ApiError(error_text));
        }

        let verify_response: VerifyResponse = response.json().await?;
        Ok(verify_response)
    }

    /// Check the health of the facilitator
    pub async fn health(&self) -> Result<HealthResponse> {
        let url = format!("{}/health", self.config.facilitator_url);

        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(QweryError::ApiError(error_text));
        }

        let health: HealthResponse = response.json().await?;
        Ok(health)
    }

    /// Get the current configuration
    pub fn config(&self) -> &QweryConfig {
        &self.config
    }
}
