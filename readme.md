# Krystal Cloud API Client

High-performance Rust client library and CLI tool for interacting with the Krystal Cloud API. Query DeFi pools, positions, and transaction data across multiple blockchain networks with type-safe async interfaces.

## Table of Contents

- [Installation](#installation)
- [Project Structure](#project-structure)
- [Quick Start](#quick-start)
- [CLI Usage](#cli-usage)
- [Library Usage](#library-usage)
- [API Examples](#api-examples)
- [Configuration](#configuration)
- [Error Handling](#error-handling)
- [Development](#development)

## Installation

### Install as Binary (macOS)

```bash
# Clone the repository
git clone https://github.com/cipher-rc5/krystal-cli.git
cd krystal-cli

# Build and install the binary
cargo install --path .

# Verify installation
krystal-cli --version
```

### Add to Your Project

Add this to your `Cargo.toml`:

```toml
[dependencies]
krystal-cli = { git = "https://github.com/cipher-rc5/krystal-cli.git" }
tokio = { version = "1.47.1", features = ["full"] }
```

## Project Structure

```
krystal-cli/
├── Cargo.toml              # Project manifest and dependencies
├── README.md               # This file
├── .env.example           # Environment variables template
├── src/
│   ├── lib.rs             # Library entry point and public API
│   ├── main.rs            # CLI application entry point
│   ├── client.rs          # Core API client implementation
│   │   ├── ClientConfig   # Configuration management
│   │   ├── KrystalApiClient # Main API client struct
│   │   └── HTTP methods   # Request/response handling
│   ├── cli/               # CLI module organization
│   │   ├── mod.rs         # CLI module exports
│   │   ├── app.rs         # CLI argument parsing and structure
│   │   │   ├── Cli        # Main CLI struct with global options
│   │   │   ├── Commands   # All available subcommands
│   │   │   └── OutputFormat # Table, JSON, CSV, Compact formats
│   │   ├── commands.rs    # Command execution logic
│   │   │   ├── Command handlers # Individual command implementations
│   │   │   └── Query builders # Transform CLI args to API queries
│   │   └── output.rs      # Output formatting and display
│   │       ├── Table formatting # Pretty table output
│   │       ├── CSV export # CSV data export
│   │       ├── JSON output # Raw JSON display
│   │       └── Detail views # Comprehensive data display
│   ├── models.rs          # Data structures and types
│   │   ├── ChainInfo      # Blockchain network information
│   │   ├── Pool           # Liquidity pool data with stats
│   │   ├── Position       # User position data with performance
│   │   ├── Transaction    # Transaction history
│   │   ├── ProtocolInfo   # Protocol metadata
│   │   ├── TokenInfo      # Token details and metadata
│   │   └── Helper types   # Stats, incentives, fees, performance
│   ├── query.rs           # Query builders and validation
│   │   ├── PoolsQuery     # Pool filtering and sorting
│   │   ├── PositionsQuery # Position filtering
│   │   └── TransactionQuery # Transaction history filters
│   ├── error.rs           # Error types and handling
│   │   ├── KrystalApiError # Comprehensive error types
│   │   ├── Retry logic    # Automatic retry detection
│   │   └── User messages  # Human-readable error messages
│   └── utils.rs           # Utility functions
│       ├── time           # Timestamp handling and formatting
│       ├── address        # Ethereum address validation
│       ├── finance        # Financial calculations and formatting
│       ├── retry          # Retry mechanisms with backoff
│       ├── pagination     # Result pagination utilities
│       └── rate_limit     # Rate limiting utilities
└── tests/                 # Comprehensive test suite
    ├── mod.rs             # Test module organization
    ├── cli_tests.rs       # CLI parsing and command tests
    ├── client_tests.rs    # API client functionality tests
    ├── query_tests.rs     # Query builder validation tests
    └── utils_tests.rs     # Utility function tests
```

## Quick Start

### Set up API Key

if you have not already done so please create a free api key at: [https://cloud.krystal.app](https://cloud.krystal.app) to use with this cli

```bash
# Option 1: Environment variable
export KRYSTAL_API_KEY="your_api_key_here"

# Option 2: Create .env file
echo "KRYSTAL_API_KEY=your_api_key_here" > .env
```

### Basic Library Usage

```rust
use krystal_cli::{KrystalApiClient, PoolsQuery, PoolSortBy};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client from environment variable
    let client = KrystalApiClient::from_env()?;

    // Get top 10 pools by TVL on Ethereum
    let pools = client.get_top_pools_by_tvl(1, 10).await?;

    for pool in pools {
        println!("{}: {}", pool.display_name(), pool.tvl);
    }

    Ok(())
}
```

## CLI Usage

### Global Options

```bash
# Output formats (can be placed before or after subcommand)
krystal-cli --format json chains           # JSON output
krystal-cli --format csv pools            # CSV output
krystal-cli --format table pools          # Table output (default)
krystal-cli --format compact pools        # Compact single-line format

# Format flag can also be used after subcommand arguments
krystal-cli pools --chain-id 1 --limit 5 --format json
krystal-cli chains --detailed --format csv

# Other global options
krystal-cli --verbose pools               # Enable debug logging
krystal-cli --no-color chains             # Disable colored output
krystal-cli --api-key "key" chains        # Override API key
```

### Chain Commands

```bash
# List all supported blockchain networks
krystal-cli chains

# Get detailed chain information
krystal-cli chains --detailed

# Filter by specific chain ID
krystal-cli chains --chain-id 1

# Get chain statistics
krystal-cli chain-stats 1                 # Ethereum stats
krystal-cli chain-stats 56                # BSC stats
```

### Pool Commands

```bash
# Basic pool queries
krystal-cli pools                          # Top 10 pools (all chains)
krystal-cli pools --chain-id 1            # Ethereum pools only
krystal-cli pools --limit 50              # Get 50 pools

# Protocol filtering
krystal-cli pools --protocol uniswapv3    # Uniswap V3 pools only
krystal-cli pools --protocol sushiv3      # SushiSwap V3 pools only

# Token filtering
krystal-cli pools --token USDC            # Pools containing USDC
krystal-cli pools --token 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48

# Advanced filtering
krystal-cli pools --chain-id 1 --protocol uniswapv3 --min-tvl 1000000 --with-incentives
krystal-cli pools --sort-by volume --min-volume 100000 --limit 20

# Detailed output
krystal-cli pools --detailed --limit 5
krystal-cli pools --format json --chain-id 1

# Pagination
krystal-cli pools --limit 20 --offset 100
```

### Pool Detail Commands

```bash
# Get detailed pool information
krystal-cli pool-detail 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640

# With factory address and incentives
krystal-cli pool-detail 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 \
    --factory 0x1f98431c8ad98523631ae4a59f267346ea31f984 \
    --with-incentives

# Get pool historical data
krystal-cli pool-history 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 \
    --days-ago 7

krystal-cli pool-history 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 \
    --start-time 1640995200 --end-time 1672531200

# Get pool transactions
krystal-cli pool-transactions 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 \
    --days-ago 1 --limit 100

krystal-cli pool-transactions 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 \
    --start-time 1640995200 --limit 50 --offset 0
```

### Position Commands

```bash
# Basic position queries
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82

# Chain-specific positions
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --chain-id 1

# Status filtering
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --status open
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --status closed
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --status all

# Protocol filtering
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 \
    --protocols uniswapv3 --protocols sushiv3

# Detailed output
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --detailed

# Multiple output formats
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --format csv
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --format json
```

### Position Detail Commands

```bash
# Get detailed position information
krystal-cli position-detail 1 0xc36442b4a4522e871399cd717abdd847ab11fe88-1028436

# Get position transaction history
krystal-cli position-transactions 1 \
    --token-address 0xC36442b4a4522E871399CD717aBDD847Ab11FE88 \
    --token-id 1028436 \
    --wallet 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 \
    --days-ago 30 \
    --limit 100
```

### Protocol Commands

```bash
# List all supported protocols
krystal-cli protocols

# Detailed protocol information
krystal-cli protocols --detailed
```

### Advanced CLI Examples

```bash
# Cross-chain portfolio analysis
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --format json > portfolio.json

# High-yield pool discovery
krystal-cli pools --sort-by apr --with-incentives --min-tvl 500000 --limit 10 --format table

# Multi-chain protocol comparison
for chain in 1 56 137; do
    echo "Chain $chain:"
    krystal-cli pools --chain-id $chain --protocol uniswapv3 --limit 5 --format compact
done

# Export pool data for analysis (format can be placed anywhere)
krystal-cli pools --chain-id 1 --limit 1000 --format csv > ethereum_pools.csv
krystal-cli --format csv pools --chain-id 1 --limit 1000 > ethereum_pools.csv

# Track specific pool performance
krystal-cli pool-detail 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 --with-incentives --format json
krystal-cli pool-transactions 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 --days-ago 1 --format table

# Position monitoring
krystal-cli positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --status open --detailed --format compact
```

### Chain IDs Reference

| Network | Chain ID | CLI Example |
|---------|----------|-------------|
| Ethereum | 1 | `--chain-id 1` |
| Optimism | 10 | `--chain-id 10` |
| Binance Smart Chain | 56 | `--chain-id 56` |
| UniChain | 130 | `--chain-id 130` |
| Polygon | 137 | `--chain-id 137` |
| Sonic | 146 | `--chain-id 146` |
| Ronin | 2020 | `--chain-id 2020` |
| Base | 8453 | `--chain-id 8453` |
| Arbitrum | 42161 | `--chain-id 42161` |
| Avalanche | 43114 | `--chain-id 43114` |
| Berachain | 80094 | `--chain-id 80094` |

## Library Usage

### Initialize Client

```rust
use krystal_cli::{KrystalApiClient, ClientConfig};

// From environment variable
let client = KrystalApiClient::from_env()?;

// With explicit API key
let client = KrystalApiClient::new("your_api_key".to_string())?;

// With custom configuration
let config = ClientConfig {
    base_url: "https://cloud-api.krystal.app".to_string(),
    timeout_secs: 60,
    user_agent: "my-app/1.0".to_string(),
};
let client = KrystalApiClient::with_config("your_key".to_string(), config)?;
```

### Query Blockchain Networks

```rust
// Get all supported chains
let chains = client.get_chains().await?;
for chain in chains {
    println!("Chain: {} (ID: {})", chain.name, chain.id);
    if let Some(protocols) = chain.supported_protocols {
        println!("  Protocols: {}", protocols.join(", "));
    }
    if let Some(explorer) = chain.explorer {
        println!("  Explorer: {}", explorer);
    }
}

// Get stats for specific chain
let stats = client.get_chain_stats(1).await?;
println!("Ethereum stats: {:#}", stats);
```

### Query Liquidity Pools

```rust
use krystal_cli::{PoolsQuery, PoolSortBy};

// Simple queries using convenience methods
let pools = client.get_top_pools_by_tvl(1, 10).await?;
let pools = client.get_top_pools_by_volume(1, 10).await?;

// Advanced filtering with query builder
let query = PoolsQuery::new()
    .chain_id(1)                    // Ethereum
    .protocol("uniswapv3")          // Uniswap V3 only
    .sort_by(PoolSortBy::Tvl)       // Sort by TVL
    .tvl_from(100000)               // Min $100k TVL
    .volume_24h_from(50000)         // Min $50k daily volume
    .with_incentives(true)          // Only pools with rewards
    .limit(50)                      // Max 50 results
    .offset(0);                     // Pagination

let pools = client.get_pools(query).await?;

// Token-specific pools
let usdc_pools = client.get_pools_for_token("USDC", Some(1)).await?;
let weth_pools = client.get_pools_for_token("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", None).await?;

// Protocol-specific pools
let uniswap_pools = client.get_pools_for_protocol("uniswapv3", Some(1), Some(20)).await?;

// Access pool data
for pool in pools {
    println!("Pool: {}", pool.display_name());
    println!("  Address: {}", pool.address);
    println!("  TVL: {}", pool.tvl);
    println!("  Pool Price: {}", pool.pool_price);

    if let Some(stats24h) = &pool.stats24h {
        println!("  24h Volume: {}", stats24h.volume);
        println!("  24h APR: {}%", stats24h.apr);
        println!("  24h Fees: {}", stats24h.fee);
    }

    if pool.is_high_activity() {
        println!("  High activity pool!");
    }
}
```

### Query User Positions

```rust
use krystal_cli::{PositionsQuery, PositionStatus};

let wallet = "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82";

// Simple position queries using convenience methods
let open_positions = client.get_open_positions(wallet, Some(1)).await?;
let closed_positions = client.get_closed_positions(wallet, Some(1)).await?;
let all_positions = client.get_all_positions(wallet, None).await?;

// Advanced position filtering
let query = PositionsQuery::new(wallet)
    .chain_id(1)
    .status(PositionStatus::Open)
    .add_protocol("uniswapv3")
    .add_protocol("sushiv3");

let positions = client.get_positions(query).await?;

// Access position data
for position in positions {
    println!("Position ID: {}", position.id);
    println!("  Current Value: {}", position.current_position_value);
    println!("  Status: {}", position.status);
    println!("  Price Range: {} - {}", position.min_price, position.max_price);

    if let Some(chain) = &position.chain {
        println!("  Chain: {} ({})", chain.name, chain.id);
    }

    if position.is_active() {
        println!("  Position is active");
    } else if position.is_closed() {
        println!("  Position is closed");
    }

    if let Some(performance) = &position.performance {
        println!("  P&L: {}", performance.pnl);
        println!("  ROI: {}%", performance.return_on_investment);
        println!("  Impermanent Loss: {}", performance.impermanent_loss);

        if let Some(apr) = &performance.apr {
            println!("  Total APR: {}%", apr.total_apr);
            println!("  Fee APR: {}%", apr.fee_apr);
            println!("  Farm APR: {}%", apr.farm_apr);
        }
    }
}
```

### Query Pool Details and Transactions

```rust
use krystal_cli::TransactionQuery;

// Get detailed pool information
let pool_detail = client.get_pool_detail(
    1,  // Ethereum
    "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640",  // USDC/WETH pool
    None,  // Factory address (optional)
    true   // Include incentives
).await?;

println!("Pool: {}", pool_detail.display_name());
println!("TVL: {}", pool_detail.tvl);
println!("Fee Tier: {}bps", pool_detail.fee_tier);

if let Some(stats24h) = &pool_detail.stats24h {
    println!("24h Volume: {}", stats24h.volume);
    println!("24h APR: {}%", stats24h.apr);
}

if let Some(incentives) = &pool_detail.incentives {
    for incentive in incentives {
        println!("Incentive: {} - {} per day",
                 incentive.incentive_type,
                 incentive.daily_reward_usd);
    }
}

// Get pool transaction history
let query = TransactionQuery::new()
    .limit(100)
    .start_time(crate::utils::time::days_ago(7)); // Last 7 days

let transactions = client.get_pool_transactions(
    1,
    "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
    None,
    Some(query)
).await?;

for tx in transactions {
    println!("TX: {} - {} ({} ago)",
             &tx.hash[..10],
             tx.transaction_type,
             if tx.is_recent() { "recent" } else { "older" });
    println!("  Amount0: {}, Amount1: {}", tx.amount0, tx.amount1);
}
```

## API Examples

### Example 1: Find Best Yield Opportunities

```rust
use krystal_cli::*;

async fn find_best_yields() -> Result<()> {
    let client = KrystalApiClient::from_env()?;

    // Get high-TVL pools with incentives across multiple chains
    let chains = [1, 56, 137]; // Ethereum, BSC, Polygon

    for chain_id in chains {
        let query = PoolsQuery::new()
            .chain_id(chain_id)
            .sort_by(PoolSortBy::Apr)
            .tvl_from(1_000_000)  // Min $1M TVL
            .with_incentives(true)
            .limit(5);

        let pools = client.get_pools(query).await?;

        println!("\nChain {}: Top 5 incentivized pools", chain_id);
        for pool in pools {
            if let Some(stats) = &pool.stats24h {
                println!("  {}: {:.2}% APR, ${:.1}M TVL",
                        pool.display_name(),
                        stats.apr,
                        pool.tvl / 1_000_000.0);
            }
        }
    }

    Ok(())
}
```

### Example 2: Portfolio Analysis

```rust
async fn analyze_portfolio(wallet: &str) -> Result<()> {
    let client = KrystalApiClient::from_env()?;

    // Get all positions across all chains
    let positions = client.get_all_positions(wallet, None).await?;

    let mut total_value = 0.0;
    let mut total_pnl = 0.0;
    let mut open_positions = 0;

    println!("Portfolio Analysis for {}", crate::utils::address::format_address_default(wallet));
    println!("{}", "=".repeat(60));

    for position in positions {
        total_value += position.current_position_value;

        if position.is_active() {
            open_positions += 1;
        }

        if let Some(perf) = &position.performance {
            total_pnl += perf.pnl;

            println!("\nPosition {}", position.id);
            if let Some(chain) = &position.chain {
                println!("  Chain: {}", chain.name);
            }
            println!("  Value: {}", crate::utils::finance::format_usd(position.current_position_value));
            println!("  P&L: {}", crate::utils::finance::format_usd(perf.pnl));
            println!("  ROI: {}", crate::utils::finance::format_percentage(perf.return_on_investment));

            if let Some(apr) = &perf.apr {
                println!("  Current APR: {}", crate::utils::finance::format_percentage(apr.total_apr));
            }
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("Summary:");
    println!("  Total Positions: {}", positions.len());
    println!("  Open Positions: {}", open_positions);
    println!("  Total Portfolio Value: {}", crate::utils::finance::format_usd(total_value));
    println!("  Total P&L: {}", crate::utils::finance::format_usd(total_pnl));

    if total_value > 0.0 {
        println!("  Overall ROI: {}", crate::utils::finance::format_percentage((total_pnl / total_value) * 100.0));
    }

    Ok(())
}
```

### Example 3: Protocol Performance Comparison

```rust
async fn compare_protocols() -> Result<()> {
    let client = KrystalApiClient::from_env()?;

    let protocols = ["uniswapv3", "sushiv3", "pancakev3"];

    println!("{:<15} {:<12} {:<12} {:<12} {:<8}", "Protocol", "Pools", "Total TVL", "Avg Volume", "Avg APR");
    println!("{}", "-".repeat(70));

    for protocol in protocols {
        let pools = client.get_pools_for_protocol(protocol, Some(1), Some(100)).await?;

        let total_tvl: f64 = pools.iter().map(|p| p.tvl).sum();
        let avg_volume: f64 = pools.iter()
            .map(|p| p.volume_24h())
            .sum::<f64>() / pools.len().max(1) as f64;
        let avg_apr: f64 = pools.iter()
            .filter_map(|p| p.apr())
            .sum::<f64>() / pools.len().max(1) as f64;

        println!("{:<15} {:<12} {:<12} {:<12} {:<8.1}%",
                protocol,
                pools.len(),
                crate::utils::finance::format_usd(total_tvl),
                crate::utils::finance::format_usd(avg_volume),
                avg_apr
        );
    }

    Ok(())
}
```

## Configuration

### Client Configuration

```rust
use krystal_cli::{ClientConfig, KrystalApiClient};

let config = ClientConfig {
    base_url: "https://cloud-api.krystal.app".to_string(),
    timeout_secs: 60,  // 60 second timeout
    user_agent: "my-defi-app/2.0".to_string(),
};

let client = KrystalApiClient::with_config("api_key".to_string(), config)?;
```

### Environment Variables

```bash
# Required
KRYSTAL_API_KEY=your_api_key_here

# Optional
RUST_LOG=debug                              # Enable debug logging
```

## Error Handling

### Comprehensive Error Types

```rust
use krystal_cli::{KrystalApiError, Result};

match client.get_pools(query).await {
    Ok(pools) => {
        println!("Found {} pools", pools.len());
        for pool in pools {
            println!("  {}: {}", pool.display_name(), pool.tvl);
        }
    }
    Err(KrystalApiError::AuthError) => {
        eprintln!("Authentication failed. Check your API key.");
        eprintln!("Set KRYSTAL_API_KEY environment variable or use --api-key option");
    }
    Err(KrystalApiError::PaymentRequired) => {
        eprintln!("No credits remaining. Please top up your account.");
    }
    Err(KrystalApiError::InvalidParams(msg)) => {
        eprintln!("Invalid request parameters: {}", msg);
    }
    Err(e) if e.is_retryable() => {
        eprintln!("Temporary error, consider retrying: {}", e.user_message());
    }
    Err(e) => eprintln!("API error: {}", e.user_message()),
}
```

### Retry Logic

```rust
use krystal_cli::utils::retry::{retry_with_backoff, RetryConfig};

let config = RetryConfig::default();
let result = retry_with_backoff(config, || async {
    client.get_chains().await
}).await?;
```

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/your-username/krystal-cli.git
cd krystal-cli

# Setup development environment
cp .env.example .env
# Edit .env and add your KRYSTAL_API_KEY

# Run tests
cargo test

# Build in release mode
cargo build --release

# Install locally
cargo install --path .

# Run CLI directly during development
cargo run -- chains
cargo run -- pools --chain-id 1 --limit 5
cargo run -- positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test modules
cargo test cli_tests
cargo test client_tests
cargo test query_tests
cargo test utils_tests

# Run tests for specific functionality
cargo test test_pools_query_builder
cargo test test_address_validation
cargo test test_usd_formatting

# Integration tests (requires API key)
KRYSTAL_API_KEY=your_key cargo test --features integration
```

### CLI Development Commands

```bash
# Test CLI parsing without API calls
cargo test cli_tests

# Test specific CLI commands during development
cargo run -- --verbose pools --chain-id 1 --limit 3
cargo run -- --format json chains
cargo run -- positions 0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82 --format compact

# Debug output formatting
cargo run -- pools --chain-id 1 --detailed --limit 2
cargo run -- pool-detail 1 0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640 --with-incentives
```

### Project Commands

```bash
# Format code
cargo fmt

# Check for issues
cargo clippy

# Generate documentation
cargo doc --open

# Check dependencies
cargo tree

# Update dependencies
cargo update

# Build optimized release
cargo build --release --locked
```

### Binary Installation Details

After installation with `cargo install --path .`, the binary will be available at:
- **macOS**: `~/.cargo/bin/krystal-cli`
- **Ensure** `~/.cargo/bin` is in your PATH:

```bash
# Add to ~/.zshrc or ~/.bash_profile
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Verify installation
which krystal-cli
krystal-cli --version
```

To uninstall:
```bash
cargo uninstall krystal-cli
```

### API Rate Limits and Costs

- **Chains API**: 0 units (free)
- **Protocols API**: 0 units (free)
- **Pools API**: 2 units per request
- **Positions API**: 10 units per request
- **Pool Details**: 2 units per request
- **Position Details**: 10 units per request

### Supported Protocols

**Uniswap Family**: `uniswapv2`, `uniswapv3`, `uniswapv4`
**SushiSwap Family**: `sushiv2`, `sushiv3`
**PancakeSwap Family**: `pancakev2`, `pancakev3`, `pancakev4`
**QuickSwap**: `quickswapv2`, `quickswapv3`
**Camelot**: `camelotv2`, `camelotv3`
**Concentrated Liquidity**: `aerodromecl`, `shadowcl`, `swapxcl`, `kodiakcl`
**Others**: `katanav2`, `katanav3`, `thena`, `wagmiv3`

### Output Format Examples

**Table Format (Default):**
```
Found 3 pools
#    Pool                 Protocol        TVL          24h Volume   24h APR
--------------------------------------------------------------------------
1    USDC/WETH           uniswapv3       $99.6M       $144M        26.4%
2    ETH/USDT            sushiv3         $45.2M       $23M         15.8%
```

**Compact Format:**
```
USDC/WETH (Uniswap V3) - TVL: $99.6M
ETH/USDT (SushiSwap V3) - TVL: $45.2M
```

**JSON Format:**
```json
[
  {
    "address": "0x88e6a0c2ddd26feeb64f039a2c41296fcb3f5640",
    "tvl": 99602546.357321,
    "protocol": {
      "name": "Uniswap V3",
      "key": "uniswapv3"
    }
  }
]
```

**CSV Format:**
```csv
index,token_pair,protocol,tvl,volume_24h,apr_24h
1,USDC/WETH,uniswapv3,99602546.357321,143948485.934915,26.375428240880527
```

### Getting Help

```bash
# General help
krystal-cli --help

# Command-specific help
krystal-cli pools --help
krystal-cli positions --help
krystal-cli pool-detail --help
krystal-cli pool-transactions --help

# List all available commands
krystal-cli --help | grep -A 20 "SUBCOMMANDS"

# Version information
krystal-cli --version
```

### Troubleshooting

**Common Issues:**

1. **Authentication Error**: Ensure `KRYSTAL_API_KEY` is set correctly
2. **Invalid Chain ID**: Use supported chain IDs from the reference table
3. **Invalid Protocol**: Use protocol keys from the supported protocols list
4. **Invalid Address**: Ensure Ethereum addresses are 42 characters starting with 0x
5. **Rate Limiting**: The client has built-in retry logic for temporary failures

**Debug Mode:**
```bash
# Enable verbose logging for debugging
krystal-cli --verbose pools --chain-id 1 --limit 3

# Enable Rust debug logging
RUST_LOG=debug krystal-cli chains
```

## Credits

thank you to krystal.app for awesome tooling and to UnityCode333 for the DeFi education

- [https://krystal.app/](https://krystal.app/)
- [api_documentation](https://cloud-api.krystal.app/swagger/index.html#/protocols/get_v1_protocols)

## Author

**ℭ𝔦𝔭𝔥𝔢𝔯**

- github: [https://github.com/cipher-rc5](https://github.com/cipher-rc5)
- x/twitter: [https://x.com/Cipher0091](https://x.com/Cipher0091)
- telegram: [@cipher0091](https://t.me/cipher0091)

## License

MIT License - see LICENSE file for details.
