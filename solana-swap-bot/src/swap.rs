use anyhow::{anyhow, Result};
use log::{debug, error, info, warn};
use std::{sync::Arc, time::Duration};

use crate::jupiter::{JupiterClient, QuoteResponse};

pub struct SwapManager {
    jupiter_client: JupiterClient,
}

impl SwapManager {
    pub fn new(jupiter_client: JupiterClient) -> Self {
        Self { jupiter_client }
    }

    pub async fn get_quote_and_route(
        &self,
        input_mint: String,
        output_mint: String,
        amount: u64,
        slippage_bps: u16,
        quote_only: bool,
    ) -> Result<String> {
        info!("Starting quote request");

        // Validate input mint address format (basic check)
        if input_mint.len() < 32 || input_mint.len() > 44 {
            return Err(anyhow!("Invalid input mint address format"));
        }

        if output_mint.len() < 32 || output_mint.len() > 44 {
            return Err(anyhow!("Invalid output mint address format"));
        }

        // Get quote from Jupiter
        let quote = self
            .jupiter_client
            .get_quote(input_mint, output_mint, amount, slippage_bps)
            .await?;

        self.log_quote_details(&quote);

        if quote_only {
            return Ok(format!(
                "Quote: {} {} -> {} {} (Price Impact: {}%)",
                quote.in_amount,
                quote.input_mint,
                quote.out_amount,
                quote.output_mint,
                quote.price_impact_pct
            ));
        }

        // For demo purposes, we'll get the swap transaction data
        // but won't actually execute it since we don't have real wallet integration
        let demo_pubkey = "11111111111111111111111111111112".to_string();
        
        match self
            .jupiter_client
            .get_swap_transaction(quote, demo_pubkey, Some(1000))
            .await
        {
            Ok(swap_response) => {
                info!("Swap transaction prepared successfully");
                Ok(format!(
                    "Swap transaction prepared (length: {} chars)",
                    swap_response.swap_transaction.len()
                ))
            }
            Err(e) => {
                warn!("Failed to get swap transaction (expected in demo mode): {}", e);
                Ok("Quote completed successfully - transaction preparation skipped in demo mode".to_string())
            }
        }
    }

    fn log_quote_details(&self, quote: &QuoteResponse) {
        info!("=== Swap Quote Details ===");
        info!("Input: {} {}", quote.in_amount, quote.input_mint);
        info!("Output: {} {}", quote.out_amount, quote.output_mint);
        info!("Price Impact: {}%", quote.price_impact_pct);
        info!("Slippage: {} bps", quote.slippage_bps);
        
        if let Some(platform_fee) = &quote.platform_fee {
            info!("Platform Fee: {} ({} bps)", platform_fee.amount, platform_fee.fee_bps);
        }

        info!("Route Plan:");
        for (i, route) in quote.route_plan.iter().enumerate() {
            info!(
                "  {}. {} ({}%) - {} {} -> {} {}",
                i + 1,
                route.swap_info.label,
                route.percent,
                route.swap_info.in_amount,
                route.swap_info.input_mint,
                route.swap_info.out_amount,
                route.swap_info.output_mint
            );
        }
        info!("========================");
    }
}