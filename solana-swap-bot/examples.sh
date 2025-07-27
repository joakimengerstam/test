#!/bin/bash

# Example usage of the Solana Token Swap Bot
# Make sure to build the project first: cargo build --release

echo "=== Solana Token Swap Bot Examples ==="
echo

# Set up logging
export RUST_LOG=info

# Example 1: Simulate swapping 0.1 SOL to USDC
echo "Example 1: Simulate swapping 0.1 SOL to USDC"
./target/debug/solana-swap-bot \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 100000000 \
  --simulate

echo
echo "=================================="
echo

# Example 2: Simulate swapping 10 USDC to SOL with higher slippage
echo "Example 2: Simulate swapping 10 USDC to SOL (higher slippage)"
./target/debug/solana-swap-bot \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint So11111111111111111111111111111111111111112 \
  --amount 10000000 \
  --slippage 100 \
  --simulate

echo
echo "=================================="
echo

# Example 3: Get swap transaction preparation (demo mode)
echo "Example 3: Prepare swap transaction for 0.05 SOL to USDC"
./target/debug/solana-swap-bot \
  --input-mint So11111111111111111111111111111111111111112 \
  --output-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --amount 50000000

echo
echo "=================================="
echo

# Example 4: Try with other tokens (simulate USDC to USDT)
echo "Example 4: Simulate swapping 5 USDC to USDT"
./target/debug/solana-swap-bot \
  --input-mint EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v \
  --output-mint Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB \
  --amount 5000000 \
  --simulate

echo
echo "All examples completed!"
echo
echo "Note: This is a demo version. To use with real transactions:"
echo "1. Add full Solana SDK dependencies"
echo "2. Load your actual wallet keypair"
echo "3. Add transaction signing and sending logic"
echo "4. Add proper error handling and retry mechanisms"