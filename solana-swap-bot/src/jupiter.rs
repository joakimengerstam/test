use anyhow::{anyhow, Result};
use log::{debug, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteRequest {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    pub amount: u64,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: u16,
    #[serde(rename = "swapMode", skip_serializing_if = "Option::is_none")]
    pub swap_mode: Option<String>,
    #[serde(rename = "onlyDirectRoutes", skip_serializing_if = "Option::is_none")]
    pub only_direct_routes: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteResponse {
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "outAmount")]
    pub out_amount: String,
    #[serde(rename = "otherAmountThreshold")]
    pub other_amount_threshold: String,
    #[serde(rename = "swapMode")]
    pub swap_mode: String,
    #[serde(rename = "slippageBps")]
    pub slippage_bps: u16,
    #[serde(rename = "platformFee")]
    pub platform_fee: Option<PlatformFee>,
    #[serde(rename = "priceImpactPct")]
    pub price_impact_pct: String,
    #[serde(rename = "routePlan")]
    pub route_plan: Vec<RoutePlan>,
    #[serde(rename = "contextSlot")]
    pub context_slot: u64,
    #[serde(rename = "timeTaken")]
    pub time_taken: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformFee {
    pub amount: String,
    #[serde(rename = "feeBps")]
    pub fee_bps: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoutePlan {
    #[serde(rename = "swapInfo")]
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapInfo {
    #[serde(rename = "ammKey")]
    pub amm_key: String,
    pub label: String,
    #[serde(rename = "inputMint")]
    pub input_mint: String,
    #[serde(rename = "outputMint")]
    pub output_mint: String,
    #[serde(rename = "inAmount")]
    pub in_amount: String,
    #[serde(rename = "outAmount")]
    pub out_amount: String,
    #[serde(rename = "feeAmount")]
    pub fee_amount: String,
    #[serde(rename = "feeMint")]
    pub fee_mint: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapRequest {
    #[serde(rename = "quoteResponse")]
    pub quote_response: QuoteResponse,
    #[serde(rename = "userPublicKey")]
    pub user_public_key: String,
    #[serde(rename = "wrapAndUnwrapSol", skip_serializing_if = "Option::is_none")]
    pub wrap_and_unwrap_sol: Option<bool>,
    #[serde(rename = "useSharedAccounts", skip_serializing_if = "Option::is_none")]
    pub use_shared_accounts: Option<bool>,
    #[serde(rename = "feeAccount", skip_serializing_if = "Option::is_none")]
    pub fee_account: Option<String>,
    #[serde(rename = "computeUnitPriceMicroLamports", skip_serializing_if = "Option::is_none")]
    pub compute_unit_price_micro_lamports: Option<u64>,
    #[serde(rename = "prioritizationFeeLamports", skip_serializing_if = "Option::is_none")]
    pub prioritization_fee_lamports: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    #[serde(rename = "swapTransaction")]
    pub swap_transaction: String,
    #[serde(rename = "lastValidBlockHeight")]
    pub last_valid_block_height: u64,
    #[serde(rename = "prioritizationFeeLamports")]
    pub prioritization_fee_lamports: Option<u64>,
    #[serde(rename = "computeUnitLimit")]
    pub compute_unit_limit: Option<u64>,
    #[serde(rename = "prioritizationType")]
    pub prioritization_type: Option<HashMap<String, serde_json::Value>>,
}

pub struct JupiterClient {
    client: Client,
    base_url: String,
}

impl JupiterClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn get_quote(
        &self,
        input_mint: String,
        output_mint: String,
        amount: u64,
        slippage_bps: u16,
    ) -> Result<QuoteResponse> {
        debug!("Requesting quote for {} {} -> {} {}", amount, input_mint, "X", output_mint);

        let url = format!("{}/quote", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&[
                ("inputMint", &input_mint),
                ("outputMint", &output_mint),
                ("amount", &amount.to_string()),
                ("slippageBps", &slippage_bps.to_string()),
                ("swapMode", &"ExactIn".to_string()),
                ("onlyDirectRoutes", &"false".to_string()),
            ])
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send quote request: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Quote request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let quote_response: QuoteResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse quote response: {}", e))?;

        info!(
            "Got quote: {} {} -> {} {} (price impact: {}%)",
            quote_response.in_amount,
            input_mint,
            quote_response.out_amount,
            output_mint,
            quote_response.price_impact_pct
        );

        Ok(quote_response)
    }

    pub async fn get_swap_transaction(
        &self,
        quote_response: QuoteResponse,
        user_public_key: String,
        priority_fee: Option<u64>,
    ) -> Result<SwapResponse> {
        let swap_request = SwapRequest {
            quote_response,
            user_public_key,
            wrap_and_unwrap_sol: Some(true),
            use_shared_accounts: Some(true),
            fee_account: None,
            compute_unit_price_micro_lamports: priority_fee,
            prioritization_fee_lamports: priority_fee,
        };

        debug!("Requesting swap transaction");

        let url = format!("{}/swap", self.base_url);
        let response = self
            .client
            .post(&url)
            .json(&swap_request)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to send swap request: {}", e))?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(anyhow!(
                "Swap request failed with status {}: {}",
                status,
                error_text
            ));
        }

        let swap_response: SwapResponse = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse swap response: {}", e))?;

        info!("Received swap transaction");

        Ok(swap_response)
    }
}