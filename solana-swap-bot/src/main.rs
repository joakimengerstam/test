use anyhow::{anyhow, Result};
use clap::{Arg, Command};
use log::{error, info, warn};
use std::{str::FromStr, sync::Arc, time::Duration};

mod jupiter;
mod wallet;
mod swap;

use jupiter::JupiterClient;
use wallet::WalletManager;
use swap::SwapManager;

#[derive(Debug, Clone)]
pub struct Config {
    pub jupiter_url: String,
    pub slippage_bps: u16,
    pub priority_fee: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jupiter_url: "https://quote-api.jup.ag/v6".to_string(),
            slippage_bps: 50, // 0.5%
            priority_fee: 1000, // microlamports
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("Solana Swap Bot")
        .version("1.0")
        .author("Your Name")
        .about("A bot for swapping tokens on Solana using Jupiter")
        .arg(
            Arg::new("input-mint")
                .short('i')
                .long("input-mint")
                .value_name("MINT")
                .help("Input token mint address")
                .required(true),
        )
        .arg(
            Arg::new("output-mint")
                .short('o')
                .long("output-mint")
                .value_name("MINT")
                .help("Output token mint address")
                .required(true),
        )
        .arg(
            Arg::new("amount")
                .short('a')
                .long("amount")
                .value_name("AMOUNT")
                .help("Amount to swap (in input token's smallest unit)")
                .required(true),
        )
        .arg(
            Arg::new("slippage")
                .short('s')
                .long("slippage")
                .value_name("BPS")
                .help("Slippage tolerance in basis points (default: 50 = 0.5%)")
                .default_value("50"),
        )
        .arg(
            Arg::new("simulate")
                .long("simulate")
                .help("Only get quote information, don't prepare swap transaction")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        jupiter_url: "https://quote-api.jup.ag/v6".to_string(),
        slippage_bps: matches
            .get_one::<String>("slippage")
            .unwrap()
            .parse()
            .map_err(|_| anyhow!("Invalid slippage value"))?,
        priority_fee: 1000,
    };

    let input_mint = matches.get_one::<String>("input-mint").unwrap();
    let output_mint = matches.get_one::<String>("output-mint").unwrap();
    let amount: u64 = matches
        .get_one::<String>("amount")
        .unwrap()
        .parse()
        .map_err(|_| anyhow!("Invalid amount"))?;
    let simulate = matches.get_flag("simulate");

    info!("Starting Solana Swap Bot");
    info!("Input mint: {}", input_mint);
    info!("Output mint: {}", output_mint);
    info!("Amount: {}", amount);
    info!("Slippage: {} bps", config.slippage_bps);
    info!("Simulate only: {}", simulate);

    // Initialize components
    let jupiter_client = JupiterClient::new(&config.jupiter_url);
    let swap_manager = SwapManager::new(jupiter_client);

    // Execute swap or get quote
    match swap_manager
        .get_quote_and_route(
            input_mint.clone(),
            output_mint.clone(),
            amount,
            config.slippage_bps,
            simulate,
        )
        .await
    {
        Ok(result) => {
            if simulate {
                info!("Quote retrieved successfully!");
                info!("Result: {}", result);
            } else {
                info!("Swap transaction prepared successfully!");
                info!("Note: This is a demo version. For actual swaps, you'd need to:");
                info!("1. Add Solana SDK dependencies");
                info!("2. Load your wallet");
                info!("3. Sign and send the transaction");
                info!("Transaction data: {}", result);
            }
        }
        Err(e) => {
            error!("Operation failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
