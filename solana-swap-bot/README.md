# Solana Token Swap Bot

A Rust-based bot for swapping tokens on the Solana blockchain using Jupiter aggregator.

## 🚀 Current Status: Working Demo

This implementation successfully demonstrates core functionality including:
- ✅ Real-time quote fetching from Jupiter API
- ✅ Detailed route analysis and price impact calculation  
- ✅ CLI interface with comprehensive options
- ✅ Simulation mode for safe testing
- ✅ Robust error handling and logging

**Note**: This is currently a demo version. For production use, additional components are needed (see [IMPLEMENTATION.md](IMPLEMENTATION.md) for details).

## Features

- **Token Swaps**: Swap any SPL tokens using Jupiter aggregator
- **Balance Checking**: Automatically checks token balances before swapping
- **Simulation Mode**: Test swaps without executing them
- **Slippage Control**: Configurable slippage tolerance
- **Priority Fees**: Support for priority fees to improve transaction success rate
- **Comprehensive Logging**: Detailed logging for debugging and monitoring
- **Route Information**: Shows detailed swap routes and price impact

## Prerequisites

- Rust (latest stable version)
- A Solana wallet with some SOL for transaction fees
- Tokens you want to swap (for the input token)

## Installation

1. Clone and build the project:
```bash
cd solana-swap-bot
cargo build --release
```

2. Set up your wallet:
```bash
# If you don't have a Solana keypair, generate one:
solana-keygen new --outfile ~/.config/solana/id.json

# Or use an existing keypair file
```

## Usage

### Basic Swap

```bash
# Swap 1000000 USDC to SOL (amounts are in token's smallest unit)
cargo run -- \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint So11111111111111111111111111111111111111112 \
  --amount 1000000 \
  --slippage 50
```

### Simulate Before Swapping

```bash
# Simulate the swap without executing it
cargo run -- \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint So11111111111111111111111111111111111111112 \
  --amount 1000000 \
  --simulate
```

### Use Custom RPC and Keypair

```bash
cargo run -- \
  --rpc-url https://api.mainnet-beta.solana.com \
  --keypair ./my-wallet.json \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint So11111111111111111111111111111111111111112 \
  --amount 1000000 \
  --slippage 100 \
  --priority-fee 2000
```

## Command Line Options

- `--rpc-url`: Solana RPC endpoint (default: mainnet-beta)
- `--keypair`: Path to keypair file (default: ~/.config/solana/id.json)
- `--input-mint`: Input token mint address (required)
- `--output-mint`: Output token mint address (required)
- `--amount`: Amount to swap in token's smallest unit (required)
- `--slippage`: Slippage tolerance in basis points (default: 50 = 0.5%)
- `--priority-fee`: Priority fee in microlamports (default: 1000)
- `--simulate`: Only simulate the swap, don't execute

## Common Token Addresses

| Token | Mint Address |
|-------|-------------|
| SOL | So11111111111111111111111111111111111111112 |
| USDC | EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v |
| USDT | Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB |
| RAY | 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R |
| SRM | SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt |

## Token Amount Calculation

Remember that token amounts should be specified in the token's smallest unit:

- **SOL**: 1 SOL = 1,000,000,000 lamports
- **USDC**: 1 USDC = 1,000,000 (6 decimals)
- **USDT**: 1 USDT = 1,000,000 (6 decimals)

## Examples

### Swap 0.1 SOL to USDC
```bash
cargo run -- \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 100000000
```

### Swap 10 USDC to RAY
```bash
cargo run -- \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint 4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R \
  --amount 10000000
```

## Safety Features

- **Balance Verification**: Checks if you have sufficient tokens before swapping
- **Simulation Support**: Test swaps without risking real tokens
- **Transaction Confirmation**: Waits for transaction confirmation with timeout
- **Error Handling**: Comprehensive error messages for troubleshooting

## Troubleshooting

### Common Issues

1. **"Insufficient balance"**: Make sure you have enough tokens and SOL for fees
2. **"Token account does not exist"**: Create an associated token account first
3. **"Transaction failed"**: Try increasing slippage or priority fee
4. **"RPC error"**: Check your RPC endpoint or try a different one

### Enable Debug Logging

```bash
RUST_LOG=debug cargo run -- [your arguments]
```

## Architecture

The bot consists of several modules:

- **main.rs**: CLI interface and application orchestration
- **wallet.rs**: Wallet management and keypair handling
- **jupiter.rs**: Jupiter API integration for quotes and swap transactions
- **swap.rs**: Swap execution and transaction management

## Security Notes

- **Never share your private key**: Keep your keypair file secure
- **Test with small amounts**: Always test with small amounts first
- **Use simulation mode**: Simulate swaps before executing them
- **Verify addresses**: Double-check token mint addresses before swapping

## Contributing

Feel free to submit issues and pull requests to improve the bot.

## License

This project is open source and available under the MIT License.