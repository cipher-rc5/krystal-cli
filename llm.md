# Directory Structure
```
src/
  cli/
    app.rs
    commands.rs
    mod.rs
    output.rs
  client.rs
  error.rs
  lib.rs
  main.rs
  models.rs
  query.rs
  utils.rs
```

# Files

## File: src/cli/app.rs
````rust
use clap::{Parser, Subcommand};
use crate::cli::commands;
use crate::error::Result;
use crate::KrystalApiClient;

#[derive(Parser)]
#[command(name = "krystal-cli")]
#[command(about = "Command line tool for interacting with the Krystal Cloud API")]
#[command(version = "0.1.0")]
#[command(long_about = "comprehensive commandline tool for querying DeFi pools, positions, transactions, and blockchain data through the Krystal Cloud API")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// API key (can also be set via KRYSTAL_API_KEY env var)
    #[arg(short, long)]
    pub api_key: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Output format
    #[arg(long, value_enum, default_value = "table")]
    pub format: OutputFormat,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List supported blockchain networks
    #[command(alias = "chain")]
    Chains {
        /// Show detailed chain information
        #[arg(short, long)]
        detailed: bool,

        /// Filter by chain ID
        #[arg(short = 'i', long)]
        chain_id: Option<u32>,

        /// Output format (overrides global setting)
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Query liquidity pools
    #[command(alias = "pool")]
    Pools {
        /// Chain ID to filter by
        #[arg(short, long)]
        chain_id: Option<u32>,

        /// Number of results to return
        #[arg(short, long, default_value = "10")]
        limit: u32,

        /// Protocol to filter by
        #[arg(short, long)]
        protocol: Option<String>,

        /// Token address to filter by
        #[arg(short, long)]
        token: Option<String>,

        /// Factory address to filter by
        #[arg(short, long)]
        factory: Option<String>,

        /// Sort by criteria
        #[arg(short, long, value_enum)]
        sort_by: Option<PoolSortBy>,

        /// Minimum TVL threshold
        #[arg(long)]
        min_tvl: Option<u32>,

        /// Minimum 24h volume threshold
        #[arg(long)]
        min_volume: Option<u32>,

        /// Show pools with incentives only
        #[arg(long)]
        with_incentives: bool,

        /// Show detailed pool information
        #[arg(short, long)]
        detailed: bool,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: u32,

        /// Output format (overrides global setting)
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Get detailed information about a specific pool
    #[command(name = "pool-detail")]
    PoolDetail {
        /// Chain ID
        chain_id: u32,

        /// Pool address
        pool_address: String,

        /// Factory address (optional)
        #[arg(short, long)]
        factory: Option<String>,

        /// Include incentives information
        #[arg(short, long)]
        with_incentives: bool,
    },

    /// Get historical data for a specific pool
    #[command(name = "pool-history")]
    PoolHistory {
        /// Chain ID
        chain_id: u32,

        /// Pool address
        pool_address: String,

        /// Factory address (optional)
        #[arg(short, long)]
        factory: Option<String>,

        /// Start timestamp (Unix timestamp)
        #[arg(long)]
        start_time: Option<u64>,

        /// End timestamp (Unix timestamp)
        #[arg(long)]
        end_time: Option<u64>,

        /// Number of days ago to start from (alternative to start_time)
        #[arg(long)]
        days_ago: Option<u64>,
    },

    /// Get transactions for a specific pool
    #[command(name = "pool-transactions")]
    PoolTransactions {
        /// Chain ID
        chain_id: u32,

        /// Pool address
        pool_address: String,

        /// Factory address (optional)
        #[arg(short, long)]
        factory: Option<String>,

        /// Start timestamp (Unix timestamp)
        #[arg(long)]
        start_time: Option<u64>,

        /// End timestamp (Unix timestamp)
        #[arg(long)]
        end_time: Option<u64>,

        /// Number of days ago to start from
        #[arg(long)]
        days_ago: Option<u64>,

        /// Maximum number of transactions to return
        #[arg(short, long, default_value = "50")]
        limit: u32,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: u32,
    },

    /// Query positions for a wallet
    #[command(alias = "pos")]
    Positions {
        /// Wallet address
        wallet: String,

        /// Chain ID to filter by
        #[arg(short, long)]
        chain_id: Option<u32>,

        /// Position status filter
        #[arg(short, long, value_enum)]
        status: Option<PositionStatusArg>,

        /// Protocols to filter by (can be specified multiple times)
        #[arg(short, long)]
        protocols: Vec<String>,

        /// Show detailed position information
        #[arg(short, long)]
        detailed: bool,

        /// Output format (overrides global setting)
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Get detailed information about a specific position
    #[command(name = "position-detail")]
    PositionDetail {
        /// Chain ID
        chain_id: u32,

        /// Position ID
        position_id: String,
    },

    /// Get transaction history for a specific position
    #[command(name = "position-transactions")]
    PositionTransactions {
        /// Chain ID
        chain_id: u32,

        /// Wallet address (optional)
        #[arg(short, long)]
        wallet: Option<String>,

        /// Token address
        token_address: String,

        /// Token ID (optional)
        #[arg(long)]
        token_id: Option<String>,

        /// Start timestamp (Unix timestamp)
        #[arg(long)]
        start_time: Option<u64>,

        /// End timestamp (Unix timestamp)
        #[arg(long)]
        end_time: Option<u64>,

        /// Number of days ago to start from
        #[arg(long)]
        days_ago: Option<u64>,

        /// Maximum number of transactions to return
        #[arg(short, long, default_value = "50")]
        limit: u32,
    },

    /// List all supported protocols
    Protocols {
        /// Show detailed protocol information
        #[arg(short, long)]
        detailed: bool,

        /// Output format (overrides global setting)
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },

    /// Get chain statistics
    #[command(name = "chain-stats")]
    ChainStats {
        /// Chain ID
        chain_id: u32,

        /// Output format (overrides global setting)
        #[arg(long, value_enum)]
        format: Option<OutputFormat>,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    /// Tabular output (default)
    Table,
    /// JSON output
    Json,
    /// CSV output
    Csv,
    /// Compact single-line format
    Compact,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PoolSortBy {
    /// Sort by Annual Percentage Rate
    Apr,
    /// Sort by Total Value Locked
    Tvl,
    /// Sort by 24-hour volume
    Volume,
    /// Sort by fees
    Fee,
}

impl From<PoolSortBy> for crate::models::PoolSortBy {
    fn from(sort: PoolSortBy) -> Self {
        match sort {
            PoolSortBy::Apr => crate::models::PoolSortBy::Apr,
            PoolSortBy::Tvl => crate::models::PoolSortBy::Tvl,
            PoolSortBy::Volume => crate::models::PoolSortBy::Volume24h,
            PoolSortBy::Fee => crate::models::PoolSortBy::Fee,
        }
    }
}

#[derive(clap::ValueEnum, Clone, Debug, PartialEq)]
pub enum PositionStatusArg {
    /// Open positions only
    Open,
    /// Closed positions only
    Closed,
    /// All positions
    All,
}

impl From<PositionStatusArg> for crate::models::PositionStatus {
    fn from(status: PositionStatusArg) -> Self {
        match status {
            PositionStatusArg::Open => crate::models::PositionStatus::Open,
            PositionStatusArg::Closed => crate::models::PositionStatus::Closed,
            PositionStatusArg::All => crate::models::PositionStatus::All,
        }
    }
}

/// Main CLI runner function
pub async fn run_cli() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    if cli.verbose {
        env_logger::init();
    }

    let client = if let Some(api_key) = cli.api_key.clone() {
        KrystalApiClient::new(api_key)?
    } else {
        KrystalApiClient::from_env()?
    };

    // Pass command and format separately to avoid borrow checker issues
    commands::execute_command(cli.command, &client, cli.format).await
}
````

## File: src/cli/commands.rs
````rust
// file: src/cli/app.rs
// description:
// docs_reference:

use crate::cli::app::PositionStatusArg;
use crate::cli::app::OutputFormat;
use crate::cli::app::Commands;
use crate::cli::output::*;
use crate::error::Result;
use crate::query::*;
use crate::utils::time;
use crate::KrystalApiClient;

/// Execute a CLI command
pub async fn execute_command(command: Commands, client: &KrystalApiClient, format: OutputFormat) -> Result<()> {
    match command {
        Commands::Chains { detailed, chain_id, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_chains(client, detailed, chain_id, effective_format).await
        }
        Commands::Pools {
            chain_id,
            limit,
            protocol,
            token,
            factory,
            sort_by,
            min_tvl,
            min_volume,
            with_incentives,
            detailed,
            offset,
            format: cmd_format,
        } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_pools(
                client,
                chain_id,
                limit,
                protocol,
                token,
                factory,
                sort_by,
                min_tvl,
                min_volume,
                with_incentives,
                detailed,
                offset,
                effective_format,
            )
            .await
        }
        Commands::PoolDetail {
            chain_id,
            pool_address,
            factory,
            with_incentives,
        } => {
            handle_pool_detail(client, chain_id, &pool_address, factory.as_deref(), with_incentives, &format)
                .await
        }
        Commands::PoolHistory {
            chain_id,
            pool_address,
            factory,
            start_time,
            end_time,
            days_ago,
        } => {
            handle_pool_history(
                client,
                chain_id,
                &pool_address,
                factory.as_deref(),
                start_time,
                end_time,
                days_ago,
                &format,
            )
            .await
        }
        Commands::PoolTransactions {
            chain_id,
            pool_address,
            factory,
            start_time,
            end_time,
            days_ago,
            limit,
            offset,
        } => {
            handle_pool_transactions(
                client,
                chain_id,
                &pool_address,
                factory.as_deref(),
                start_time,
                end_time,
                days_ago,
                limit,
                offset,
                &format,
            )
            .await
        }
        Commands::Positions {
            wallet,
            chain_id,
            status,
            protocols,
            detailed,
            format: cmd_format,
        } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_positions(client, &wallet, chain_id, status, protocols, detailed, effective_format)
                .await
        }
        Commands::PositionDetail {
            chain_id,
            position_id,
        } => handle_position_detail(client, chain_id, &position_id, &format).await,
        Commands::PositionTransactions {
            chain_id,
            wallet,
            token_address,
            token_id,
            start_time,
            end_time,
            days_ago,
            limit,
        } => {
            handle_position_transactions(
                client,
                chain_id,
                wallet.as_deref(),
                &token_address,
                token_id.as_deref(),
                start_time,
                end_time,
                days_ago,
                limit,
                &format,
            )
            .await
        }
        Commands::Protocols { detailed, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_protocols(client, detailed, effective_format).await
        }
        Commands::ChainStats { chain_id, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_chain_stats(client, chain_id, effective_format).await
        }
    }
}

async fn handle_chains(
    client: &KrystalApiClient,
    detailed: bool,
    chain_id: Option<u32>,
    format: &OutputFormat,
) -> Result<()> {
    let chains = client.get_chains().await?;

    let filtered_chains: Vec<_> = if let Some(id) = chain_id {
        chains.into_iter().filter(|c| c.id == id).collect()
    } else {
        chains
    };

    match format {
        OutputFormat::Json => print_json(&filtered_chains)?,
        OutputFormat::Csv => print_chains_csv(&filtered_chains, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_chains_table(&filtered_chains, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_pools(
    client: &KrystalApiClient,
    chain_id: Option<u32>,
    limit: u32,
    protocol: Option<String>,
    token: Option<String>,
    factory: Option<String>,
    sort_by: Option<crate::cli::app::PoolSortBy>,
    min_tvl: Option<u32>,
    min_volume: Option<u32>,
    with_incentives: bool,
    detailed: bool,
    offset: u32,
    format: &OutputFormat,
) -> Result<()> {
    let mut query = PoolsQuery::new().limit(limit).offset(offset);

    if let Some(cid) = chain_id {
        query = query.chain_id(cid);
    }
    if let Some(proto) = protocol {
        query = query.protocol(proto);
    }
    if let Some(token_addr) = token {
        query = query.token(token_addr);
    }
    if let Some(factory_addr) = factory {
        query = query.factory_address(factory_addr);
    }
    if let Some(sort) = sort_by {
        query = query.sort_by(sort.into());
    }
    if let Some(tvl) = min_tvl {
        query = query.min_tvl(tvl);
    }
    if let Some(volume) = min_volume {
        query = query.min_volume_24h(volume);
    }
    if with_incentives {
        query = query.with_incentives(true);
    }

    let pools = client.get_pools(query).await?;

    match format {
        OutputFormat::Json => print_json(&pools)?,
        OutputFormat::Csv => print_pools_csv(&pools, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_pools_table(&pools, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_pool_detail(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    with_incentives: bool,
    format: &OutputFormat,
) -> Result<()> {
    let pool = client
        .get_pool_detail(chain_id, pool_address, factory_address, with_incentives)
        .await?;

    match format {
        OutputFormat::Json => print_json(&pool)?,
        _ => print_pool_detail(&pool)?,
    }

    Ok(())
}

async fn handle_pool_history(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, None, None);

    let history = client
        .get_pool_historical(chain_id, pool_address, factory_address, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&history)?,
        _ => {
            println!("Historical data for pool {}:", pool_address);
            print_json(&history)?;
        }
    }

    Ok(())
}

async fn handle_pool_transactions(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: u32,
    offset: u32,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, Some(limit), Some(offset));

    let transactions = client
        .get_pool_transactions(chain_id, pool_address, factory_address, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&transactions)?,
        OutputFormat::Csv => print_transactions_csv(&transactions)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_transactions_table(&transactions, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_positions(
    client: &KrystalApiClient,
    wallet: &str,
    chain_id: Option<u32>,
    status: Option<PositionStatusArg>,
    protocols: Vec<String>,
    detailed: bool,
    format: &OutputFormat,
) -> Result<()> {
    let mut query = PositionsQuery::new(wallet);

    if let Some(cid) = chain_id {
        query = query.chain_id(cid);
    }
    if let Some(status_arg) = status {
        query = query.status(status_arg.into());
    }
    if !protocols.is_empty() {
        query = query.protocols(protocols);
    }

    let positions = client.get_positions(query).await?;

    match format {
        OutputFormat::Json => print_json(&positions)?,
        OutputFormat::Csv => print_positions_csv(&positions, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_positions_table(&positions, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_position_detail(
    client: &KrystalApiClient,
    chain_id: u32,
    position_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let position = client.get_position_detail(chain_id, position_id).await?;

    match format {
        OutputFormat::Json => print_json(&position)?,
        _ => print_position_detail(&position)?,
    }

    Ok(())
}

async fn handle_position_transactions(
    client: &KrystalApiClient,
    chain_id: u32,
    wallet: Option<&str>,
    token_address: &str,
    token_id: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: u32,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, Some(limit), None);

    let transactions = client
        .get_position_transactions(chain_id, wallet, token_address, token_id, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&transactions)?,
        OutputFormat::Csv => print_transactions_csv(&transactions)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_transactions_table(&transactions, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_protocols(
    client: &KrystalApiClient,
    detailed: bool,
    format: &OutputFormat,
) -> Result<()> {
    let protocols = client.get_protocols().await?;

    match format {
        OutputFormat::Json => print_json(&protocols)?,
        _ => {
            println!("Supported Protocols:");
            if detailed {
                print_json(&protocols)?;
            } else {
                // Try to extract protocol names from the response
                if let Some(protocols_array) = protocols.as_array() {
                    for (i, protocol) in protocols_array.iter().enumerate() {
                        if let Some(name) = protocol.get("name").and_then(|n| n.as_str()) {
                            println!("{}. {}", i + 1, name);
                        } else if let Some(key) = protocol.get("key").and_then(|k| k.as_str()) {
                            println!("{}. {}", i + 1, key);
                        }
                    }
                } else {
                    print_json(&protocols)?;
                }
            }
        }
    }

    Ok(())
}

async fn handle_chain_stats(
    client: &KrystalApiClient,
    chain_id: u32,
    format: &OutputFormat,
) -> Result<()> {
    let stats = client.get_chain_stats(chain_id).await?;

    match format {
        OutputFormat::Json => print_json(&stats)?,
        _ => {
            println!("Chain {} Statistics:", chain_id);
            print_json(&stats)?;
        }
    }

    Ok(())
}

/// Helper function to build transaction query from various time parameters
fn build_transaction_query(
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Option<TransactionQuery> {
    let mut query = TransactionQuery::new();
    let mut has_params = false;

    if let Some(days) = days_ago {
        query = query.start_time(time::days_ago(days));
        has_params = true;
    } else if let Some(start) = start_time {
        query = query.start_time(start);
        has_params = true;
    }

    if let Some(end) = end_time {
        query = query.end_time(end);
        has_params = true;
    }

    if let Some(lmt) = limit {
        query = query.limit(lmt);
        has_params = true;
    }

    if let Some(off) = offset {
        query = query.offset(off);
        has_params = true;
    }

    if has_params {
        Some(query)
    } else {
        None
    }
}
````

## File: src/cli/mod.rs
````rust
// file: src/cli/mod.rs
// description:
// docs_reference:

pub mod app;
pub mod commands;
pub mod output;

pub use app::{Cli, run_cli};
pub use commands::*;
pub use output::*;
````

## File: src/cli/output.rs
````rust
// file: src/cli/output.rs
// description:
// docs_reference:

use crate::error::Result;
use crate::models::*;
use crate::utils::{address, finance};
use serde::Serialize;

/// Print data as JSON
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Print chains in table format
pub fn print_chains_table(chains: &[ChainInfo], detailed: bool, compact: bool) -> Result<()> {
    if chains.is_empty() {
        println!("No chains found");
        return Ok(());
    }

    println!("Found {} supported chains", chains.len());

    if compact {
        for chain in chains {
            println!("{}: {}", chain.id, chain.name);
        }
    } else if detailed {
        for (i, chain) in chains.iter().enumerate() {
            println!("\n{}. {} (ID: {})", i + 1, chain.name, chain.id);
            if let Some(logo) = &chain.logo {
                println!("   Logo: {}", logo);
            }
            if let Some(explorer) = &chain.explorer {
                println!("   Explorer: {}", explorer);
            }
            if !chain.additional_fields.is_empty() {
                println!("   Additional fields: {:?}", chain.additional_fields);
            }
        }
    } else {
        println!("{:<4} {:<20} {:<50}", "ID", "Name", "Explorer");
        println!("{}", "-".repeat(75));

        for chain in chains {
            let explorer = chain.explorer.as_deref().unwrap_or("N/A");
            println!(
                "{:<4} {:<20} {:<50}",
                chain.id,
                truncate_string(&chain.name, 20),
                truncate_string(explorer, 50)
            );
        }
    }

    Ok(())
}

/// Print chains in CSV format
pub fn print_chains_csv(chains: &[ChainInfo], detailed: bool) -> Result<()> {
    if detailed {
        println!("id,name,logo,explorer");
        for chain in chains {
            println!(
                "{},{},{},{}",
                chain.id,
                escape_csv(&chain.name),
                chain.logo.as_deref().unwrap_or(""),
                chain.explorer.as_deref().unwrap_or("")
            );
        }
    } else {
        println!("id,name");
        for chain in chains {
            println!("{},{}", chain.id, escape_csv(&chain.name));
        }
    }
    Ok(())
}

/// Print pools in table format
pub fn print_pools_table(pools: &[Pool], detailed: bool, compact: bool) -> Result<()> {
    if pools.is_empty() {
        println!("No pools found");
        return Ok(());
    }

    println!("Found {} pools", pools.len());

    if compact {
        for pool in pools {
            let token_pair = get_token_pair_display(&pool);
            let protocol_name = pool.protocol.as_ref()
                .map(|p| p.name.as_str())
                .unwrap_or("Unknown");
            println!("{} ({}) - TVL: {}", token_pair, protocol_name, finance::format_usd(pool.tvl));
        }
    } else if detailed {
        for (i, pool) in pools.iter().enumerate() {
            print_pool_summary(i + 1, pool)?;
        }
    } else {
        print_pools_table_header();
        for (i, pool) in pools.iter().enumerate() {
            print_pool_table_row(i + 1, pool)?;
        }
    }

    Ok(())
}

/// Print pools in CSV format
pub fn print_pools_csv(pools: &[Pool], detailed: bool) -> Result<()> {
    if detailed {
        println!("index,chain_id,chain_name,pool_address,protocol,token0_symbol,token1_symbol,fee_tier,tvl,pool_price,volume_24h,apr_24h");
        for (i, pool) in pools.iter().enumerate() {
            let chain_info = pool.chain.as_ref();
            let token0_symbol = pool.token0.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
            let token1_symbol = pool.token1.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
            let protocol_name = pool.protocol.as_ref().map_or("Unknown".to_string(), |p| p.name.clone());
            let volume_24h = pool.stats24h.as_ref().map(|s| s.volume).unwrap_or(0.0);
            let apr_24h = pool.stats24h.as_ref().map(|s| s.apr).unwrap_or(0.0);

            println!(
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                i + 1,
                chain_info.map(|c| c.id).unwrap_or(0),
                escape_csv(&chain_info.map_or("Unknown".to_string(), |c| c.name.clone())),
                escape_csv(&pool.address),
                escape_csv(&protocol_name),
                escape_csv(&token0_symbol),
                escape_csv(&token1_symbol),
                pool.fee_tier,
                pool.tvl,
                pool.pool_price,
                volume_24h,
                apr_24h
            );
        }
    } else {
        println!("index,token_pair,protocol,tvl,volume_24h,apr_24h");
        for (i, pool) in pools.iter().enumerate() {
            let token_pair = get_token_pair_display(&pool);
            let protocol_name = pool.protocol.as_ref().map_or("Unknown".to_string(), |p| p.name.clone());
            let volume_24h = pool.stats24h.as_ref().map(|s| s.volume).unwrap_or(0.0);
            let apr_24h = pool.stats24h.as_ref().map(|s| s.apr).unwrap_or(0.0);

            println!(
                "{},{},{},{},{},{}",
                i + 1,
                escape_csv(&token_pair),
                escape_csv(&protocol_name),
                pool.tvl,
                volume_24h,
                apr_24h
            );
        }
    }
    Ok(())
}

/// Print detailed pool information
pub fn print_pool_detail(pool: &Pool) -> Result<()> {
    println!("\n{}", pool.display_name());
    println!("Address: {}", pool.address);

    if let Some(chain) = &pool.chain {
        println!("Chain: {} (ID: {})", chain.name, chain.id);
        if let Some(explorer) = &chain.explorer {
            println!("Explorer: {}", explorer);
        }
    }

    if let Some(protocol) = &pool.protocol {
        println!("Protocol: {} ({})", protocol.name, protocol.key);
        println!("Factory: {}", protocol.factory_address);
    }

    println!("Fee Tier: {}bps", pool.fee_tier);
    println!("TVL: {}", finance::format_usd(pool.tvl));
    println!("Pool Price: {:.8}", pool.pool_price);

    if let Some(token0) = &pool.token0 {
        println!("Token0: {} ({}) - {}", token0.symbol, token0.name, token0.address);
    }
    if let Some(token1) = &pool.token1 {
        println!("Token1: {} ({}) - {}", token1.symbol, token1.name, token1.address);
    }

    // Statistics
    if let Some(stats1h) = &pool.stats1h {
        println!("\n1h Statistics:");
        println!("  Volume: {}", finance::format_usd(stats1h.volume));
        println!("  Fees: {}", finance::format_usd(stats1h.fee));
        println!("  APR: {}", finance::format_percentage(stats1h.apr));
    }

    if let Some(stats24h) = &pool.stats24h {
        println!("\n24h Statistics:");
        println!("  Volume: {}", finance::format_usd(stats24h.volume));
        println!("  Fees: {}", finance::format_usd(stats24h.fee));
        println!("  APR: {}", finance::format_percentage(stats24h.apr));
    }

    if let Some(stats7d) = &pool.stats7d {
        println!("\n7d Statistics:");
        println!("  Volume: {}", finance::format_usd(stats7d.volume));
        println!("  Fees: {}", finance::format_usd(stats7d.fee));
        println!("  APR: {}", finance::format_percentage(stats7d.apr));
    }

    if let Some(stats30d) = &pool.stats30d {
        println!("\n30d Statistics:");
        println!("  Volume: {}", finance::format_usd(stats30d.volume));
        println!("  Fees: {}", finance::format_usd(stats30d.fee));
        println!("  APR: {}", finance::format_percentage(stats30d.apr));
    }

    // Incentives
    if let Some(incentives) = &pool.incentives {
        if !incentives.is_empty() {
            println!("\nIncentives:");
            for incentive in incentives {
                println!("  Type: {}", incentive.incentive_type);
                println!("  Token: {} ({})", incentive.token.symbol, incentive.token.name);
                println!("  Daily Reward: {}", finance::format_usd(incentive.daily_reward_usd));
                println!("  24h APR: {}", finance::format_percentage(incentive.apr24h));
                println!();
            }
        }
    }

    Ok(())
}

/// Print positions in table format
pub fn print_positions_table(positions: &[Position], detailed: bool, compact: bool) -> Result<()> {
    if positions.is_empty() {
        println!("No positions found");
        return Ok(());
    }

    println!("Found {} positions", positions.len());

    if compact {
        for position in positions {
            println!("{} - Status: {}, Value: {}",
                position.id,
                position.status,
                finance::format_usd(position.current_position_value)
            );
        }
    } else if detailed {
        for (i, pos) in positions.iter().enumerate() {
            print_position_summary(i + 1, pos)?;
        }
    } else {
        println!("{:<4} {:<20} {:<10} {:<12} {:<10} {:<8}",
            "#", "Position ID", "Status", "Value", "Chain", "Protocol");
        println!("{}", "-".repeat(70));

        for (i, pos) in positions.iter().enumerate() {
            let chain_name = pos.chain.as_ref()
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");
            let protocol_name = pos.pool.as_ref()
                .and_then(|p| p.protocol.as_ref())
                .map(|pr| pr.name.as_str())
                .unwrap_or("Unknown");

            println!(
                "{:<4} {:<20} {:<10} {:<12} {:<10} {:<8}",
                i + 1,
                truncate_string(&pos.id, 20),
                pos.status,
                finance::format_usd(pos.current_position_value),
                truncate_string(chain_name, 10),
                truncate_string(protocol_name, 8)
            );
        }
    }

    Ok(())
}

/// Print positions in CSV format
pub fn print_positions_csv(positions: &[Position], detailed: bool) -> Result<()> {
    if detailed {
        println!("index,position_id,chain_id,chain_name,status,current_value,min_price,max_price,liquidity");
        for (i, pos) in positions.iter().enumerate() {
            let chain_info = pos.chain.as_ref();
            println!(
                "{},{},{},{},{},{},{},{},{}",
                i + 1,
                escape_csv(&pos.id),
                chain_info.map(|c| c.id).unwrap_or(0),
                escape_csv(&chain_info.map(|c| &c.name).unwrap_or(&"Unknown".to_string())),
                escape_csv(&pos.status),
                pos.current_position_value,
                pos.min_price,
                pos.max_price,
                escape_csv(&pos.liquidity)
            );
        }
    } else {
        println!("index,position_id,status,current_value");
        for (i, pos) in positions.iter().enumerate() {
            println!(
                "{},{},{},{}",
                i + 1,
                escape_csv(&pos.id),
                escape_csv(&pos.status),
                pos.current_position_value
            );
        }
    }
    Ok(())
}

/// Print detailed position information
pub fn print_position_detail(position: &Position) -> Result<()> {
    println!("\nPosition: {}", position.id);
    println!("Owner: {}", address::format_address_default(&position.owner_address));
    println!("Token Address: {}", position.token_address);
    println!("Token ID: {}", position.token_id);
    println!("Status: {}", position.status);
    println!("Liquidity: {}", position.liquidity);
    println!("Price Range: {:.6} - {:.6}", position.min_price, position.max_price);
    println!("Current Value: {}", finance::format_usd(position.current_position_value));

    if let Some(chain) = &position.chain {
        println!("Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(pool) = &position.pool {
        println!("Pool: {}", pool.pool_address);
        if let Some(protocol) = &pool.protocol {
            println!("Protocol: {} ({})", protocol.name, protocol.key);
        }
    }

    if let Some(current_amounts) = &position.current_amounts {
        println!("\nCurrent Token Amounts:");
        for amount in current_amounts {
            println!("  {}: {} ({})",
                amount.token.symbol,
                amount.balance,
                finance::format_usd(amount.value)
            );
        }
    }

    if let Some(provided_amounts) = &position.provided_amounts {
        println!("\nProvided Token Amounts:");
        for amount in provided_amounts {
            println!("  {}: {} ({})",
                amount.token.symbol,
                amount.balance,
                finance::format_usd(amount.value)
            );
        }
    }

    if let Some(performance) = &position.performance {
        println!("\nPerformance:");
        println!("  Total Deposit Value: {}", finance::format_usd(performance.total_deposit_value));
        println!("  Total Withdraw Value: {}", finance::format_usd(performance.total_withdraw_value));
        println!("  P&L: {}", finance::format_usd(performance.pnl));
        println!("  ROI: {}", finance::format_percentage(performance.return_on_investment));
        println!("  Impermanent Loss: {}", finance::format_usd(performance.impermanent_loss));

        if let Some(compare_to_hold) = performance.compare_to_hold {
            println!("  Compare to Hold: {}", finance::format_percentage(compare_to_hold));
        }

        if let Some(apr) = &performance.apr {
            println!("  Total APR: {}", finance::format_percentage(apr.total_apr));
            println!("  Fee APR: {}", finance::format_percentage(apr.fee_apr));
            println!("  Farm APR: {}", finance::format_percentage(apr.farm_apr));
        }
    }

    Ok(())
}

/// Print transactions in table format
pub fn print_transactions_table(transactions: &[Transaction], compact: bool) -> Result<()> {
    if transactions.is_empty() {
        println!("No transactions found");
        return Ok(());
    }

    println!("Found {} transactions", transactions.len());

    if compact {
        for tx in transactions {
            println!("{}: {} - {:.4}/{:.4}",
                &tx.hash[0..10],
                tx.transaction_type,
                tx.amount0,
                tx.amount1
            );
        }
    } else {
        println!("{:<12} {:<10} {:<15} {:<15} {:<20}",
            "Hash", "Type", "Amount0", "Amount1", "Time");
        println!("{}", "-".repeat(75));

        for tx in transactions {
            let time_str = crate::utils::time::format_timestamp(tx.timestamp);
            println!(
                "{:<12} {:<10} {:<15.4} {:<15.4} {:<20}",
                &tx.hash[0..10],
                truncate_string(&tx.transaction_type, 10),
                tx.amount0,
                tx.amount1,
                truncate_string(&time_str, 20)
            );
        }
    }

    Ok(())
}

/// Print transactions in CSV format
pub fn print_transactions_csv(transactions: &[Transaction]) -> Result<()> {
    println!("hash,type,amount0,amount1,timestamp");
    for tx in transactions {
        println!(
            "{},{},{},{},{}",
            escape_csv(&tx.hash),
            escape_csv(&tx.transaction_type),
            tx.amount0,
            tx.amount1,
            tx.timestamp
        );
    }
    Ok(())
}

// Helper functions

fn print_pools_table_header() {
    println!("{:<4} {:<20} {:<15} {:<12} {:<12} {:<8}",
        "#", "Pool", "Protocol", "TVL", "24h Volume", "24h APR");
    println!("{}", "-".repeat(75));
}

fn print_pool_table_row(index: usize, pool: &Pool) -> Result<()> {
    let token_pair = get_token_pair_display(&pool);
    let protocol_name = pool.protocol.as_ref()
        .map(|p| p.key.as_str())
        .unwrap_or("Unknown");
    let volume_24h = pool.stats24h.as_ref()
        .map(|s| s.volume)
        .unwrap_or(0.0);
    let apr_24h = pool.stats24h.as_ref()
        .map(|s| s.apr)
        .unwrap_or(0.0);

    println!("{:<4} {:<20} {:<15} {:<12} {:<12} {:<8.1}%",
        index,
        truncate_string(&token_pair, 20),
        truncate_string(protocol_name, 15),
        format_usd_compact(pool.tvl),
        format_usd_compact(volume_24h),
        apr_24h
    );
    Ok(())
}

fn print_pool_summary(index: usize, pool: &Pool) -> Result<()> {
    println!("\n{}. {}", index, pool.display_name());
    println!("   Address: {}", address::format_address_default(&pool.address));

    if let Some(chain) = &pool.chain {
        println!("   Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(protocol) = &pool.protocol {
        println!("   Protocol: {} ({})", protocol.name, protocol.key);
    }

    println!("   Fee Tier: {}bps", pool.fee_tier);
    println!("   TVL: {}", finance::format_usd(pool.tvl));
    println!("   Pool Price: {:.8}", pool.pool_price);

    if let Some(stats24h) = &pool.stats24h {
        println!("   24h Volume: {}", finance::format_usd(stats24h.volume));
        println!("   24h Fees: {}", finance::format_usd(stats24h.fee));
        println!("   24h APR: {}", finance::format_percentage(stats24h.apr));
    }

    if let Some(stats7d) = &pool.stats7d {
        println!("   7d APR: {}", finance::format_percentage(stats7d.apr));
    }

    Ok(())
}

fn print_position_summary(index: usize, position: &Position) -> Result<()> {
    println!("\n{}. Position {}", index, position.id);
    println!("   Owner: {}", address::format_address_default(&position.owner_address));
    println!("   Status: {}", position.status);
    println!("   Value: {}", finance::format_usd(position.current_position_value));
    println!("   Price Range: {:.6} - {:.6}", position.min_price, position.max_price);

    if let Some(chain) = &position.chain {
        println!("   Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(pool) = &position.pool {
        if let Some(protocol) = &pool.protocol {
            println!("   Protocol: {}", protocol.name);
        }
    }

    Ok(())
}

fn get_token_pair_display(pool: &Pool) -> String {
    match (&pool.token0, &pool.token1) {
        (Some(t0), Some(t1)) => format!("{}/{}", t0.symbol, t1.symbol),
        _ => "Unknown/Unknown".to_string(),
    }
}

fn format_usd_compact(amount: f64) -> String {
    if amount >= 1_000_000_000.0 {
        format!("{:.1}B", amount / 1_000_000_000.0)
    } else if amount >= 1_000_000.0 {
        format!("{:.1}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("{:.1}K", amount / 1_000.0)
    } else if amount >= 1.0 {
        format!("{:.0}", amount)
    } else {
        format!("{:.2}", amount)
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
````

## File: src/client.rs
````rust
// file: src/client.rs
// description: Main API client implementation for the Krystal Cloud API, handling HTTP requests,
//             authentication, response parsing, and providing high-level methods for API interaction
// docs_reference: https://docs.rs/reqwest/latest/reqwest/

use crate::error::{KrystalApiError, Result};
use crate::models::*;
use crate::query::*;
use reqwest::{Client, Response};
use std::env;
use url::Url;

/// Configuration for the API client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Base URL for the API
    pub base_url: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// User agent string
    pub user_agent: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "https://cloud-api.krystal.app".to_string(),
            timeout_secs: 30,
            user_agent: "krystal-rust-client/0.1.0".to_string(),
        }
    }
}

/// Main API client for interacting with the Krystal Cloud API
#[derive(Debug)]
pub struct KrystalApiClient {
    client: Client,
    config: ClientConfig,
    api_key: String,
}

impl KrystalApiClient {
    /// Create a new API client with custom configuration
    pub fn with_config(api_key: String, config: ClientConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .build()?;

        Ok(Self {
            client,
            config,
            api_key,
        })
    }

    /// Create a new API client with default configuration
    pub fn new(api_key: String) -> Result<Self> {
        Self::with_config(api_key, ClientConfig::default())
    }

    /// Create client from environment variable `KRYSTAL_API_KEY`
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("KRYSTAL_API_KEY")?;
        Self::new(api_key)
    }

    /// Handle API response and convert to appropriate error types
    async fn handle_response(response: Response) -> Result<serde_json::Value> {
        let status = response.status();

        match status.as_u16() {
            200..=299 => {
                let text = response.text().await?;
                let json: serde_json::Value = serde_json::from_str(&text)?;
                Ok(json)
            }
            400 => {
                let error_body = response.text().await.unwrap_or_default();
                Err(KrystalApiError::InvalidParams(format!(
                    "Bad request: {}",
                    error_body
                )))
            }
            401 => Err(KrystalApiError::AuthError),
            402 => Err(KrystalApiError::PaymentRequired),
            _ => {
                let error_text = response.text().await.unwrap_or_default();
                Err(KrystalApiError::ApiError {
                    status: status.as_u16(),
                    message: error_text,
                })
            }
        }
    }

    /// Create a GET request with authentication headers
    fn authenticated_get(&self, url: Url) -> reqwest::RequestBuilder {
        self.client
            .get(url)
            .header("KC-APIKey", &self.api_key)
            .header("Content-Type", "application/json")
    }

    /// Get list of all supported blockchain networks
    pub async fn get_chains(&self) -> Result<Vec<ChainInfo>> {
        let url = Url::parse(&format!("{}/v1/chains", self.config.base_url))?;
        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        // Handle both array response and object with chains field
        let chains_data = json
            .as_array()
            .or_else(|| json.get("chains").and_then(|c| c.as_array()))
            .ok_or_else(|| {
                KrystalApiError::InvalidParams("Invalid chains response format".to_string())
            })?;

        let chains: Result<Vec<ChainInfo>> = chains_data
            .iter()
            .map(|chain| serde_json::from_value(chain.clone()).map_err(KrystalApiError::from))
            .collect();

        chains
    }

    /// Get stats for a specific chain
    pub async fn get_chain_stats(&self, chain_id: u32) -> Result<serde_json::Value> {
        let url = Url::parse(&format!(
            "{}/v1/chains/{}",
            self.config.base_url, chain_id
        ))?;
        let response = self.authenticated_get(url).send().await?;
        Self::handle_response(response).await
    }

    /// Get pool data with filtering options
    pub async fn get_pools(&self, query: PoolsQuery) -> Result<Vec<Pool>> {
        // Validate query before making request
        let _ = query.validate();

        let mut url = Url::parse(&format!("{}/v1/pools", self.config.base_url))?;

        // Build query parameters
        self.build_pools_query_params(&mut url, &query);

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        // Handle both array response and object with pools field
        let empty_vec = vec![];
        let pools_data = json
            .get("pools")
            .and_then(|p| p.as_array())
            .or_else(|| json.as_array())
            .unwrap_or(&empty_vec);

        let pools: Result<Vec<Pool>> = pools_data
            .iter()
            .map(|pool| serde_json::from_value(pool.clone()).map_err(KrystalApiError::from))
            .collect();

        pools
    }

    /// Helper method to build query parameters for pools
    fn build_pools_query_params(&self, url: &mut Url, query: &PoolsQuery) {
        let mut query_pairs = url.query_pairs_mut();

        if let Some(chain_id) = query.chain_id {
            query_pairs.append_pair("chainId", &chain_id.to_string());
        }
        if let Some(ref factory_address) = query.factory_address {
            query_pairs.append_pair("factoryAddress", factory_address);
        }
        if let Some(ref protocol) = query.protocol {
            query_pairs.append_pair("protocol", protocol);
        }
        if let Some(ref token) = query.token {
            query_pairs.append_pair("token", token);
        }
        if let Some(sort_by) = query.sort_by {
            query_pairs.append_pair("sortBy", &u8::from(sort_by).to_string());
        }
        if let Some(tvl_from) = query.tvl_from {
            query_pairs.append_pair("tvlFrom", &tvl_from.to_string());
        }
        if let Some(volume_from) = query.volume_24h_from {
            query_pairs.append_pair("volume24hFrom", &volume_from.to_string());
        }
        if let Some(limit) = query.limit {
            query_pairs.append_pair("limit", &limit.to_string());
        }
        if let Some(offset) = query.offset {
            query_pairs.append_pair("offset", &offset.to_string());
        }
        if let Some(with_incentives) = query.with_incentives {
            query_pairs.append_pair("withIncentives", &with_incentives.to_string());
        }
    }

    /// Get detailed information about a specific pool
    pub async fn get_pool_detail(
        &self,
        chain_id: u32,
        pool_address: &str,
        factory_address: Option<&str>,
        with_incentives: bool,
    ) -> Result<Pool> {
        let mut url = Url::parse(&format!(
            "{}/v1/pools/{}/{}",
            self.config.base_url, chain_id, pool_address
        ))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(factory) = factory_address {
                query_pairs.append_pair("factoryAddress", factory);
            }
            query_pairs.append_pair("withIncentives", &with_incentives.to_string());
        }

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        // Parse as Pool directly
        serde_json::from_value(json).map_err(KrystalApiError::from)
    }

    /// Get historical data for a specific pool
    pub async fn get_pool_historical(
        &self,
        chain_id: u32,
        pool_address: &str,
        factory_address: Option<&str>,
        query: Option<TransactionQuery>,
    ) -> Result<serde_json::Value> {
        let mut url = Url::parse(&format!(
            "{}/v1/pools/{}/{}/historical",
            self.config.base_url, chain_id, pool_address
        ))?;

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(factory) = factory_address {
                query_pairs.append_pair("factoryAddress", factory);
            }

            if let Some(ref q) = query {
                if let Some(start) = q.start_time {
                    query_pairs.append_pair("startTime", &start.to_string());
                }
                if let Some(end) = q.end_time {
                    query_pairs.append_pair("endTime", &end.to_string());
                }
            }
        }

        let response = self.authenticated_get(url).send().await?;
        Self::handle_response(response).await
    }

    /// Get transactions for a specific pool
    pub async fn get_pool_transactions(
        &self,
        chain_id: u32,
        pool_address: &str,
        factory_address: Option<&str>,
        query: Option<TransactionQuery>,
    ) -> Result<Vec<Transaction>> {
        let mut url = Url::parse(&format!(
            "{}/v1/pools/{}/{}/transactions",
            self.config.base_url, chain_id, pool_address
        ))?;

        {
            let mut query_pairs = url.query_pairs_mut();

            if let Some(factory) = factory_address {
                query_pairs.append_pair("factoryAddress", factory);
            }

            if let Some(ref q) = query {
                if let Some(start) = q.start_time {
                    query_pairs.append_pair("startTime", &start.to_string());
                }
                if let Some(end) = q.end_time {
                    query_pairs.append_pair("endTime", &end.to_string());
                }
                if let Some(limit) = q.limit {
                    query_pairs.append_pair("limit", &limit.to_string());
                }
                if let Some(offset) = q.offset {
                    query_pairs.append_pair("offset", &offset.to_string());
                }
            }
        }

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        let empty_vec = vec![];
        let txs_data = json
            .get("transactions")
            .and_then(|t| t.as_array())
            .unwrap_or(&empty_vec);

        let transactions: Result<Vec<Transaction>> = txs_data
            .iter()
            .map(|tx| serde_json::from_value(tx.clone()).map_err(KrystalApiError::from))
            .collect();

        transactions
    }

    /// Get all positions for a wallet
    pub async fn get_positions(&self, query: PositionsQuery) -> Result<Vec<Position>> {
        // Validate query before making request
        let _ = query.validate();

        let mut url = Url::parse(&format!("{}/v1/positions", self.config.base_url))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("wallet", &query.wallet);

            if let Some(chain_id) = query.chain_id {
                query_pairs.append_pair("chainId", &chain_id.to_string());
            }
            if let Some(ref status) = query.position_status {
                if let Some(status_str) = status.as_str() {
                    query_pairs.append_pair("positionStatus", status_str);
                }
            }
            if let Some(ref protocols) = query.protocols {
                for protocol in protocols {
                    query_pairs.append_pair("protocols", protocol);
                }
            }
        }

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        let empty_vec = vec![];
        let positions_data = json
            .get("positions")
            .and_then(|p| p.as_array())
            .or_else(|| json.as_array())
            .unwrap_or(&empty_vec);

        let positions: Result<Vec<Position>> = positions_data
            .iter()
            .map(|pos| serde_json::from_value(pos.clone()).map_err(KrystalApiError::from))
            .collect();

        positions
    }

    /// Get detailed information about a specific position
    pub async fn get_position_detail(
        &self,
        chain_id: u32,
        position_id: &str,
    ) -> Result<Position> {
        let url = Url::parse(&format!(
            "{}/v1/positions/{}/{}",
            self.config.base_url, chain_id, position_id
        ))?;

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        // Parse as Position directly
        serde_json::from_value(json).map_err(KrystalApiError::from)
    }

    /// Get transaction history for a specific position
    pub async fn get_position_transactions(
        &self,
        chain_id: u32,
        wallet: Option<&str>,
        token_address: &str,
        token_id: Option<&str>,
        query: Option<TransactionQuery>,
    ) -> Result<Vec<Transaction>> {
        if let Some(q) = &query {
            let _ = q.validate();
        }

        let mut url = Url::parse(&format!(
            "{}/v1/positions/{}/transactions",
            self.config.base_url, chain_id
        ))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("tokenAddress", token_address);

            if let Some(wallet) = wallet {
                query_pairs.append_pair("wallet", wallet);
            }
            if let Some(token_id) = token_id {
                query_pairs.append_pair("tokenId", token_id);
            }

            if let Some(ref q) = query {
                if let Some(start) = q.start_time {
                    query_pairs.append_pair("startTimestamp", &start.to_string());
                }
                if let Some(end) = q.end_time {
                    query_pairs.append_pair("endTimestamp", &end.to_string());
                }
                if let Some(limit) = q.limit {
                    query_pairs.append_pair("limit", &limit.to_string());
                }
            }
        }

        let response = self.authenticated_get(url).send().await?;
        let json = Self::handle_response(response).await?;

        let empty_vec = vec![];
        let txs_data = json
            .get("transactions")
            .and_then(|t| t.as_array())
            .unwrap_or(&empty_vec);

        let transactions: Result<Vec<Transaction>> = txs_data
            .iter()
            .map(|tx| serde_json::from_value(tx.clone()).map_err(KrystalApiError::from))
            .collect();

        transactions
    }

    /// Get list of all supported protocols
    pub async fn get_protocols(&self) -> Result<serde_json::Value> {
        let url = Url::parse(&format!("{}/v1/protocols", self.config.base_url))?;
        let response = self.authenticated_get(url).send().await?;
        Self::handle_response(response).await
    }
}

// Convenience methods for common use cases
impl KrystalApiClient {
    /// Get top pools by TVL for a specific chain
    pub async fn get_top_pools_by_tvl(&self, chain_id: u32, limit: u32) -> Result<Vec<Pool>> {
        let query = PoolsQuery::new()
            .chain_id(chain_id)
            .sort_by(PoolSortBy::Tvl)
            .limit(limit);

        self.get_pools(query).await
    }

    /// Get top pools by 24h volume for a specific chain
    pub async fn get_top_pools_by_volume(&self, chain_id: u32, limit: u32) -> Result<Vec<Pool>> {
        let query = PoolsQuery::new()
            .chain_id(chain_id)
            .sort_by(PoolSortBy::Volume24h)
            .limit(limit);

        self.get_pools(query).await
    }

    /// Get pools for a specific token
    pub async fn get_pools_for_token(
        &self,
        token: &str,
        chain_id: Option<u32>,
    ) -> Result<Vec<Pool>> {
        let mut query = PoolsQuery::new().token(token);

        if let Some(cid) = chain_id {
            query = query.chain_id(cid);
        }

        self.get_pools(query).await
    }

    /// Get pools for a specific protocol
    pub async fn get_pools_for_protocol(
        &self,
        protocol: &str,
        chain_id: Option<u32>,
        limit: Option<u32>,
    ) -> Result<Vec<Pool>> {
        let mut query = PoolsQuery::new().protocol(protocol);

        if let Some(cid) = chain_id {
            query = query.chain_id(cid);
        }
        if let Some(lmt) = limit {
            query = query.limit(lmt);
        }

        self.get_pools(query).await
    }

    /// Get open positions for a wallet
    pub async fn get_open_positions(
        &self,
        wallet: &str,
        chain_id: Option<u32>,
    ) -> Result<Vec<Position>> {
        let mut query = PositionsQuery::new(wallet).status(PositionStatus::Open);

        if let Some(cid) = chain_id {
            query = query.chain_id(cid);
        }

        self.get_positions(query).await
    }

    /// Get closed positions for a wallet
    pub async fn get_closed_positions(
        &self,
        wallet: &str,
        chain_id: Option<u32>,
    ) -> Result<Vec<Position>> {
        let mut query = PositionsQuery::new(wallet).status(PositionStatus::Closed);

        if let Some(cid) = chain_id {
            query = query.chain_id(cid);
        }

        self.get_positions(query).await
    }

    /// Get all positions for a wallet (open and closed)
    pub async fn get_all_positions(
        &self,
        wallet: &str,
        chain_id: Option<u32>,
    ) -> Result<Vec<Position>> {
        let mut query = PositionsQuery::new(wallet).status(PositionStatus::All);

        if let Some(cid) = chain_id {
            query = query.chain_id(cid);
        }

        self.get_positions(query).await
    }

    /// Get recent transactions for a pool
    pub async fn get_recent_pool_transactions(
        &self,
        chain_id: u32,
        pool_address: &str,
        limit: u32,
    ) -> Result<Vec<Transaction>> {
        let query = TransactionQuery::new().limit(limit);
        self.get_pool_transactions(chain_id, pool_address, None, Some(query))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = KrystalApiClient::new("test-key".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_with_custom_config() {
        let config = ClientConfig {
            base_url: "https://api.example.com".to_string(),
            timeout_secs: 60,
            user_agent: "test-client/1.0".to_string(),
        };

        let client = KrystalApiClient::with_config("test-key".to_string(), config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.base_url, "https://cloud-api.krystal.app");
        assert_eq!(config.timeout_secs, 30);
    }

    #[test]
    fn test_chain_id_formatting() {
        let mut url = Url::parse("https://api.example.com/test").unwrap();
        let query = PoolsQuery::new().chain_id(1);

        let client = KrystalApiClient::new("test".to_string()).unwrap();
        client.build_pools_query_params(&mut url, &query);

        let query_string = url.query().unwrap_or("");
        assert!(query_string.contains("chainId=1"));
    }
}
````

## File: src/error.rs
````rust
// file: src/error.rs
// description: Error types and handling for the Krystal API client, providing comprehensive
//             error categorization with retryability detection and user-friendly messages
// docs_reference: https://docs.rs/thiserror/latest/thiserror/

use reqwest::Error as ReqwestError;
use thiserror::Error;

/// Custom error types for better error handling and debugging.
#[derive(Error, Debug)]
pub enum KrystalApiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] ReqwestError),

    /// API returned an error response
    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Authentication failed
    #[error("Authentication failed: Missing or invalid API key")]
    AuthError,

    /// Payment required - no credit left
    #[error("Payment required: No credit left")]
    PaymentRequired,

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, KrystalApiError>;

impl KrystalApiError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RequestError(_)
                | Self::ApiError {
                    status: 500..=599,
                    ..
                }
        )
    }

    /// Check if error is related to authentication
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError)
    }

    /// Check if error requires payment
    pub fn requires_payment(&self) -> bool {
        matches!(self, Self::PaymentRequired)
    }

    /// Get user-friendly error message with suggested actions
    pub fn user_message(&self) -> String {
        match self {
            Self::AuthError => {
                "Authentication failed. Please check your API key is correct and has proper permissions.".to_string()
            }
            Self::PaymentRequired => {
                "Your account has no remaining credits. Please top up your balance to continue.".to_string()
            }
            Self::RequestError(e) if e.is_timeout() => {
                "Request timed out. Please try again or check your internet connection.".to_string()
            }
            Self::RequestError(e) if e.is_connect() => {
                "Could not connect to the API. Please check your internet connection.".to_string()
            }
            Self::InvalidParams(msg) => {
                format!("Invalid request parameters: {}", msg)
            }
            _ => self.to_string(),
        }
    }
}
````

## File: src/lib.rs
````rust
//! # Krystal Cloud API Client
//!
//! A Rust client library for interacting with the Krystal Cloud API.
//! This library provides a type-safe, async interface for querying DeFi pools,
//! positions, and transaction data across multiple blockchain networks.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use krystal_cli::{KrystalApiClient, PoolsQuery};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = KrystalApiClient::from_env()?;
//!     let pools = client.get_top_pools_by_tvl(1, 10).await?;
//!     println!("Found {} pools", pools.len());
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod query;
pub mod utils;

pub mod cli;

pub use client::{ClientConfig, KrystalApiClient};
pub use error::{KrystalApiError, Result};
pub use models::{
    ChainInfo, PaginatedResponse, Pool, PoolSortBy, Position, PositionStatus, Transaction,
};
pub use query::{PoolsQuery, PositionsQuery, TransactionQuery};


pub use cli::app::run_cli;
````

## File: src/main.rs
````rust
// file: src/main.rs
// description: Command-line interface for the Krystal API client, providing interactive
//             commands for querying pools, positions, and blockchain data with formatted output
// docs_reference: https://docs.rs/clap/latest/clap/

use krystal_cli::run_cli;
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run_cli().await {
        eprintln!("Error: {}", e);
        eprintln!("Suggestion: {}", e.user_message());
        process::exit(1);
    }
}
````

## File: src/models.rs
````rust
// file: src/models.rs
// description: Data models and structures for the Krystal API client, defining typed
//             representations of chains, pools, positions, and transactions with serde support
// docs_reference: https://docs.rs/serde/latest/serde/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a blockchain network supported by Krystal
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ChainInfo {
    /// Unique identifier for the chain
    pub id: u32,
    /// Human-readable name of the chain
    pub name: String,
    /// Logo URL for the chain
    pub logo: Option<String>,
    /// Chain explorer URL
    pub explorer: Option<String>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Information about a liquidity pool - matches actual API response
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Pool {
    /// Chain information
    pub chain: Option<ChainInfo>,
    /// Contract address of the pool
    #[serde(rename = "poolAddress")]
    pub address: String,
    /// Pool price (token0 in terms of token1)
    #[serde(rename = "poolPrice")]
    pub pool_price: f64,  // Changed from Option<String> to f64
    /// Protocol information
    pub protocol: Option<ProtocolInfo>,
    /// Fee tier in basis points
    #[serde(rename = "feeTier")]
    pub fee_tier: u32,
    /// First token in the pair
    pub token0: Option<TokenInfo>,
    /// Second token in the pair
    pub token1: Option<TokenInfo>,
    /// Total Value Locked in USD
    pub tvl: f64,
    /// 1-hour statistics
    pub stats1h: Option<PoolStats>,
    /// 24-hour statistics
    pub stats24h: Option<PoolStats>,
    /// 7-day statistics
    pub stats7d: Option<PoolStats>,
    /// 30-day statistics
    pub stats30d: Option<PoolStats>,
    /// Incentives information
    pub incentives: Option<Vec<IncentiveInfo>>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Protocol information
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct ProtocolInfo {
    /// Protocol key
    pub key: String,
    /// Protocol display name
    pub name: String,
    /// Factory contract address
    #[serde(rename = "factoryAddress")]
    pub factory_address: String,
    /// Protocol logo URL
    pub logo: Option<String>,
}

/// Token information
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TokenInfo {
    /// Token contract address
    pub address: String,
    /// Token symbol
    pub symbol: String,
    /// Token name
    pub name: String,
    /// Token decimals
    pub decimals: u8,
    /// Token logo URL
    pub logo: Option<String>,
}

/// Pool statistics
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PoolStats {
    /// Trading volume in USD
    pub volume: f64,
    /// Fees collected in USD
    pub fee: f64,
    /// Annual Percentage Rate
    pub apr: f64,
}

/// Incentive information
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct IncentiveInfo {
    /// Type of incentive
    #[serde(rename = "incentiveType")]
    pub incentive_type: String,
    /// Reward token details
    pub token: TokenInfo,
    /// Amount distributed per day
    #[serde(rename = "amountPerDay")]
    pub amount_per_day: f64,
    /// Daily reward value in USD
    #[serde(rename = "dailyRewardUsd")]
    pub daily_reward_usd: f64,
    /// 24-hour APR from rewards
    pub apr24h: f64,
}

/// Information about a liquidity position
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Position {
    /// Unique identifier for the position
    pub id: String,
    /// Chain information
    pub chain: Option<ChainInfo>,
    /// Pool information
    pub pool: Option<PoolInfo>,
    /// Owner wallet address
    #[serde(rename = "ownerAddress")]
    pub owner_address: String,
    /// NFT token address
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    /// NFT token ID
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// Position liquidity
    pub liquidity: String,
    /// Minimum price range
    #[serde(rename = "minPrice")]
    pub min_price: f64,
    /// Maximum price range
    #[serde(rename = "maxPrice")]
    pub max_price: f64,
    /// Current position value in USD
    #[serde(rename = "currentPositionValue")]
    pub current_position_value: f64,
    /// Status of the position
    pub status: String,
    /// Current token amounts
    #[serde(rename = "currentAmounts")]
    pub current_amounts: Option<Vec<TokenWithValue>>,
    /// Originally provided amounts
    #[serde(rename = "providedAmounts")]
    pub provided_amounts: Option<Vec<TokenWithValue>>,
    /// Trading fee information
    #[serde(rename = "tradingFee")]
    pub trading_fee: Option<FeeInfo>,
    /// Farming reward information
    #[serde(rename = "farmingReward")]
    pub farming_reward: Option<FeeInfo>,
    /// Performance metrics
    pub performance: Option<PositionPerformance>,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Pool information for positions
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PoolInfo {
    /// Pool ID/address
    pub id: String,
    /// Pool address
    #[serde(rename = "poolAddress")]
    pub pool_address: String,
    /// Protocol information
    pub protocol: Option<ProtocolInfo>,
}

/// Token with value information
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TokenWithValue {
    /// Token details
    pub token: TokenInfo,
    /// Token balance
    pub balance: String,
    /// Token price in USD
    pub price: f64,
    /// Total value in USD
    pub value: f64,
}

/// Fee information (pending and claimed)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FeeInfo {
    /// Pending fees
    pub pending: Option<Vec<TokenWithValue>>,
    /// Claimed fees
    pub claimed: Option<Vec<TokenWithValue>>,
}

/// Position performance metrics
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct PositionPerformance {
    /// Total deposited value in USD
    #[serde(rename = "totalDepositValue")]
    pub total_deposit_value: f64,
    /// Total withdrawn value in USD
    #[serde(rename = "totalWithdrawValue")]
    pub total_withdraw_value: f64,
    /// Impermanent loss
    #[serde(rename = "impermanentLoss")]
    pub impermanent_loss: f64,
    /// Profit and loss
    pub pnl: f64,
    /// Return on investment
    #[serde(rename = "returnOnInvestment")]
    pub return_on_investment: f64,
    /// Comparison to holding
    #[serde(rename = "compareToHold")]
    pub compare_to_hold: Option<f64>,
    /// APR breakdown
    pub apr: Option<AprBreakdown>,
}

/// APR breakdown
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct AprBreakdown {
    /// Total APR
    #[serde(rename = "totalApr")]
    pub total_apr: f64,
    /// Fee APR
    #[serde(rename = "feeApr")]
    pub fee_apr: f64,
    /// Farming APR
    #[serde(rename = "farmApr")]
    pub farm_apr: f64,
}

/// Information about a transaction
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Transaction {
    /// Transaction hash
    pub hash: String,
    /// Unix timestamp of the transaction
    pub timestamp: u64,
    /// Type of transaction (e.g., "swap", "mint", "burn")
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Amount of token0 involved
    pub amount0: f64,
    /// Amount of token1 involved
    pub amount1: f64,
    /// Additional fields that might be present in the API response
    #[serde(flatten)]
    pub additional_fields: HashMap<String, serde_json::Value>,
}

/// Response wrapper for paginated results
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    /// The actual data items
    pub data: Vec<T>,
    /// Total number of items available
    pub total: Option<u64>,
    /// Current page/offset
    pub offset: Option<u64>,
    /// Number of items per page
    pub limit: Option<u64>,
    /// Whether there are more items available
    pub has_more: Option<bool>,
}

/// Sort options for pools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PoolSortBy {
    /// Sort by Annual Percentage Rate
    Apr = 0,
    /// Sort by Total Value Locked
    Tvl = 1,
    /// Sort by 24-hour volume
    Volume24h = 2,
    /// Sort by fees
    Fee = 3,
}

impl From<PoolSortBy> for u8 {
    fn from(sort: PoolSortBy) -> Self {
        sort as u8
    }
}

/// Position status options
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PositionStatus {
    /// Position is currently open
    Open,
    /// Position has been closed
    Closed,
    /// All positions regardless of status
    All,
}

impl PositionStatus {
    /// Convert to string for API calls
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Self::Open => Some("OPEN"),
            Self::Closed => Some("CLOSED"),
            Self::All => None,
        }
    }
}

impl Pool {
    /// Calculate volume-to-TVL ratio
    pub fn volume_tvl_ratio(&self) -> f64 {
        if self.tvl > 0.0 {
            if let Some(stats) = &self.stats24h {
                stats.volume / self.tvl
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Check if pool has high activity (volume >= 10% of TVL)
    pub fn is_high_activity(&self) -> bool {
        self.volume_tvl_ratio() >= 0.1
    }

    /// Format pool for display
    /// Format pool for display
    pub fn display_name(&self) -> String {
        let token0_symbol = self.token0.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
        let token1_symbol = self.token1.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
        let protocol_name = self.protocol.as_ref().map_or("Unknown".to_string(), |p| p.name.clone());

        format!("{}/{} ({}) Pool", token0_symbol, token1_symbol, protocol_name)
    }

    /// Get 24h volume
    pub fn volume_24h(&self) -> f64 {
        self.stats24h.as_ref().map(|s| s.volume).unwrap_or(0.0)
    }

    /// Get 24h APR
    pub fn apr(&self) -> Option<f64> {
        self.stats24h.as_ref().map(|s| s.apr)
    }
}

impl Position {
    /// Calculate total USD value of position (approximate)
    pub fn total_value_estimate(&self, _token0_price: f64, _token1_price: f64) -> f64 {
        if let Some(amounts) = &self.current_amounts {
            amounts.iter().map(|amount| amount.value).sum()
        } else {
            // Fallback to current position value
            self.current_position_value
        }
    }

    /// Check if position is active
    pub fn is_active(&self) -> bool {
        self.status.to_uppercase() == "IN_RANGE" || self.status.to_uppercase() == "OUT_RANGE"
    }

    /// Check if position is closed
    pub fn is_closed(&self) -> bool {
        self.status.to_uppercase() == "CLOSED"
    }
}

impl Transaction {
    /// Get transaction age in seconds from current time
    pub fn age_seconds(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.timestamp)
    }

    /// Check if transaction is recent (within last hour)
    pub fn is_recent(&self) -> bool {
        self.age_seconds() < 3600
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_sort_by_conversion() {
        assert_eq!(u8::from(PoolSortBy::Apr), 0);
        assert_eq!(u8::from(PoolSortBy::Tvl), 1);
        assert_eq!(u8::from(PoolSortBy::Volume24h), 2);
        assert_eq!(u8::from(PoolSortBy::Fee), 3);
    }

    #[test]
    fn test_position_status_conversion() {
        assert_eq!(PositionStatus::Open.as_str(), Some("OPEN"));
        assert_eq!(PositionStatus::Closed.as_str(), Some("CLOSED"));
        assert_eq!(PositionStatus::All.as_str(), None);
    }

    #[test]
    fn test_chain_info_with_integer_id() {
        let json = r#"{"id": 1, "name": "Ethereum"}"#;
        let chain: ChainInfo = serde_json::from_str(json).unwrap();
        assert_eq!(chain.id, 1);
        assert_eq!(chain.name, "Ethereum");
    }

    #[test]
    fn test_pool_deserialization_with_actual_response() {
        let json = r#"{
            "chain": {
                "name": "Ethereum",
                "id": 1,
                "logo": "https://files.krystal.app/DesignAssets/chains/ethereum.png",
                "explorer": "https://etherscan.io"
            },
            "poolAddress": "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41",
            "poolPrice": 2.129633981728694,
            "protocol": {
                "key": "uniswapv3",
                "name": "Uniswap V3",
                "factoryAddress": "0x1f98431c8ad98523631ae4a59f267346ea31f984",
                "logo": "https://files.krystal.app/DesignAssets/platformIcons/uniswap.png"
            },
            "feeTier": 10000,
            "token0": {
                "address": "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
                "symbol": "USDC",
                "name": "USD Coin",
                "decimals": 6,
                "logo": "https://storage.googleapis.com/k-assets-prod.krystal.team/krystal/usd-coin.png"
            },
            "token1": {
                "address": "0xc43c6bfeda065fe2c4c11765bf838789bd0bb5de",
                "symbol": "RED",
                "name": "Redstone",
                "decimals": 18,
                "logo": "https://storage.googleapis.com/k-assets-prod.krystal.team/krystal/redstone-oracles.png"
            },
            "tvl": 7397.082829,
            "stats1h": {
                "volume": 12.264582,
                "fee": 0.122646,
                "apr": 14.524360275052842
            },
            "stats24h": {
                "volume": 65524.502322,
                "fee": 655.245021,
                "apr": 3233.226370148831
            },
            "stats7d": {
                "volume": 65530.105276,
                "fee": 655.301051,
                "apr": 119.56577410110056
            },
            "stats30d": {
                "volume": 105388.236962,
                "fee": 1053.882365,
                "apr": 84.04131140277042
            }
        }"#;

        let pool: Pool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.address, "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41");
        assert_eq!(pool.pool_price, 2.129633981728694);
        assert!(pool.chain.is_some());
        assert!(pool.protocol.is_some());
        assert!(pool.token0.is_some());
        assert!(pool.token1.is_some());

        let chain = pool.chain.unwrap();
        assert_eq!(chain.name, "Ethereum");
        assert_eq!(chain.id, 1);

        let protocol = pool.protocol.unwrap();
        assert_eq!(protocol.key, "uniswapv3");
        assert_eq!(protocol.name, "Uniswap V3");

        let token0 = pool.token0.unwrap();
        assert_eq!(token0.symbol, "USDC");
        assert_eq!(token0.decimals, 6);

        let token1 = pool.token1.unwrap();
        assert_eq!(token1.symbol, "RED");
        assert_eq!(token1.decimals, 18);
    }

    #[test]
    fn test_pool_helper_methods() {
        let json = r#"{
            "chain": {"name": "Ethereum", "id": 1, "explorer": "https://etherscan.io"},
            "poolAddress": "0x123",
            "poolPrice": 1.5,
            "protocol": {"key": "uniswapv3", "name": "Uniswap V3", "factoryAddress": "0x456"},
            "feeTier": 3000,
            "token0": {"address": "0x789", "symbol": "TOKEN0", "name": "Token 0", "decimals": 18},
            "token1": {"address": "0xabc", "symbol": "TOKEN1", "name": "Token 1", "decimals": 6},
            "tvl": 10000.0,
            "stats24h": {"volume": 1000.0, "fee": 10.0, "apr": 10.0}
        }"#;

        let pool: Pool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.display_name(), "TOKEN0/TOKEN1 (Uniswap V3) Pool");
        assert_eq!(pool.volume_24h(), 1000.0);
        assert_eq!(pool.apr(), Some(10.0));
        assert_eq!(pool.volume_tvl_ratio(), 0.1);
        assert!(pool.is_high_activity());
    }
}
````

## File: src/query.rs
````rust
// file: src/query.rs
// description: Query builders for API requests, providing fluent interfaces for constructing
//             complex filtered queries for pools, positions, and transactions with type safety
// docs_reference: https://docs.rs/url/latest/url/

use crate::models::{PoolSortBy, PositionStatus};

/// Query parameters for filtering pools
#[derive(Debug, Clone, Default)]
pub struct PoolsQuery {
    /// Filter by specific chain ID
    pub chain_id: Option<u32>,
    /// Filter by factory contract address
    pub factory_address: Option<String>,
    /// Filter by protocol name
    pub protocol: Option<String>,
    /// Filter by token address (either token0 or token1)
    pub token: Option<String>,
    /// Sort results by specified criteria
    pub sort_by: Option<PoolSortBy>,
    /// Minimum Total Value Locked threshold
    pub min_tvl: Option<u32>,
    /// Minimum 24-hour volume threshold
    pub min_volume_24h: Option<u32>,
    /// Maximum number of results to return
    pub limit: Option<u32>,
    /// Number of results to skip (for pagination)
    pub offset: Option<u32>,
    /// Include pools with incentives only
    pub with_incentives: Option<bool>,
    pub(crate) tvl_from: Option<i64>,
    pub(crate) volume_24h_from: Option<i64>,
}

impl PoolsQuery {
    /// Create a new empty query builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set chain ID filter
    pub fn chain_id(mut self, chain_id: u32) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Set factory address filter
    pub fn factory_address<S: Into<String>>(mut self, address: S) -> Self {
        self.factory_address = Some(address.into());
        self
    }

    /// Set protocol filter
    pub fn protocol<S: Into<String>>(mut self, protocol: S) -> Self {
        self.protocol = Some(protocol.into());
        self
    }

    /// Set token filter
    pub fn token<S: Into<String>>(mut self, token: S) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Set sort criteria
    pub fn sort_by(mut self, sort: PoolSortBy) -> Self {
        self.sort_by = Some(sort);
        self
    }

    /// Set minimum TVL threshold
    pub fn min_tvl(mut self, tvl: u32) -> Self {
        self.min_tvl = Some(tvl);
        self
    }

    /// Set minimum 24h volume threshold
    pub fn min_volume_24h(mut self, volume: u32) -> Self {
        self.min_volume_24h = Some(volume);
        self
    }

    /// Set result limit (pagination)
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set result offset (pagination)
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Filter to pools with incentives only
    pub fn with_incentives(mut self, enabled: bool) -> Self {
        self.with_incentives = Some(enabled);
        self
    }

    /// Validate query parameters
    pub fn validate(&self) -> Result<(), String> {
        if let Some(limit) = self.limit {
            if limit == 0 || limit > 1000 {
                return Err("Limit must be between 1 and 1000".to_string());
            }
        }

        if let Some(tvl) = self.min_tvl {
            if tvl > 1_000_000_000 {
                return Err("Minimum TVL threshold too high".to_string());
            }
        }

        Ok(())
    }
}

/// Query parameters for filtering positions
#[derive(Debug, Clone)]
pub struct PositionsQuery {
    /// Wallet address to query positions for
    pub wallet: String,
    /// Filter by specific chain ID
    pub chain_id: Option<u32>,
    /// Filter by position status
    pub position_status: Option<PositionStatus>,
    /// Filter by specific protocols
    pub protocols: Option<Vec<String>>,
}

impl PositionsQuery {
    /// Create a new positions query for a wallet
    pub fn new<S: Into<String>>(wallet: S) -> Self {
        Self {
            wallet: wallet.into(),
            chain_id: None,
            position_status: None,
            protocols: None,
        }
    }

    /// Set chain ID filter
    pub fn chain_id(mut self, chain_id: u32) -> Self {
        self.chain_id = Some(chain_id);
        self
    }

    /// Set position status filter
    pub fn status(mut self, status: PositionStatus) -> Self {
        self.position_status = Some(status);
        self
    }

    /// Set protocols filter
    pub fn protocols<I, S>(mut self, protocols: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.protocols = Some(protocols.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Add a single protocol to the filter
    pub fn add_protocol<S: Into<String>>(mut self, protocol: S) -> Self {
        self.protocols
            .get_or_insert_with(Vec::new)
            .push(protocol.into());
        self
    }

    /// Validate query parameters
    pub fn validate(&self) -> Result<(), String> {
        if self.wallet.is_empty() {
            return Err("Wallet address cannot be empty".to_string());
        }

        // Basic Ethereum address validation
        if !self.wallet.starts_with("0x") || self.wallet.len() != 42 {
            return Err("Invalid Ethereum address format".to_string());
        }

        Ok(())
    }
}

/// Query parameters for transaction history
#[derive(Debug, Clone, Default)]
pub struct TransactionQuery {
    /// Start timestamp (Unix timestamp)
    pub start_time: Option<u64>,
    /// End timestamp (Unix timestamp)
    pub end_time: Option<u64>,
    /// Maximum number of results
    pub limit: Option<u32>,
    /// Offset for pagination
    pub offset: Option<u32>,
}

impl TransactionQuery {
    /// Create a new transaction query
    pub fn new() -> Self {
        Self::default()
    }

    /// Set start time filter
    pub fn start_time(mut self, timestamp: u64) -> Self {
        self.start_time = Some(timestamp);
        self
    }

    /// Set end time filter
    pub fn end_time(mut self, timestamp: u64) -> Self {
        self.end_time = Some(timestamp);
        self
    }

    /// Set time range filter
    pub fn time_range(mut self, start: u64, end: u64) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    /// Set result limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set offset for pagination
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Validate query parameters
    pub fn validate(&self) -> Result<(), String> {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            if start >= end {
                return Err("Start time must be before end time".to_string());
            }
        }

        if let Some(limit) = self.limit {
            if limit == 0 || limit > 10000 {
                return Err("Limit must be between 1 and 10000".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pools_query_builder() {
        let query = PoolsQuery::new()
            .chain_id(1)
            .protocol("uniswapv3")
            .min_tvl(10000)
            .limit(100)
            .sort_by(PoolSortBy::Tvl);

        assert_eq!(query.chain_id, Some(1));
        assert_eq!(query.protocol, Some("uniswapv3".to_string()));
        assert_eq!(query.min_tvl, Some(10000));
        assert_eq!(query.limit, Some(100));
        assert_eq!(query.sort_by, Some(PoolSortBy::Tvl));

        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_pools_query_validation() {
        let invalid_query = PoolsQuery::new().limit(0);
        assert!(invalid_query.validate().is_err());

        let valid_query = PoolsQuery::new().limit(100);
        assert!(valid_query.validate().is_ok());
    }

    #[test]
    fn test_positions_query_builder() {
        let query = PositionsQuery::new("0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82")
            .chain_id(1)
            .status(PositionStatus::Open)
            .add_protocol("Uniswap V3")
            .add_protocol("SushiSwap");

        assert_eq!(query.wallet, "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82");
        assert_eq!(query.chain_id, Some(1));
        assert_eq!(query.position_status, Some(PositionStatus::Open));
        assert_eq!(
            query.protocols,
            Some(vec!["Uniswap V3".to_string(), "SushiSwap".to_string()])
        );

        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_positions_query_validation() {
        let invalid_query = PositionsQuery::new("invalid-address");
        assert!(invalid_query.validate().is_err());

        let valid_query = PositionsQuery::new("0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82");
        assert!(valid_query.validate().is_ok());
    }

    #[test]
    fn test_transaction_query_builder() {
        let query = TransactionQuery::new()
            .time_range(1000000, 2000000)
            .limit(50)
            .offset(10);

        assert_eq!(query.start_time, Some(1000000));
        assert_eq!(query.end_time, Some(2000000));
        assert_eq!(query.limit, Some(50));
        assert_eq!(query.offset, Some(10));

        assert!(query.validate().is_ok());
    }

    #[test]
    fn test_transaction_query_validation() {
        let invalid_query = TransactionQuery::new().time_range(2000000, 1000000); // End before start
        assert!(invalid_query.validate().is_err());

        let valid_query = TransactionQuery::new().time_range(1000000, 2000000);
        assert!(valid_query.validate().is_ok());
    }
}
````

## File: src/utils.rs
````rust
// file: src/utils.rs
// description: Utility functions and helpers for the Krystal API client, including time handling,
//             address validation, financial formatting, and retry logic for robust API interactions
// docs_reference: https://docs.rs/tokio/latest/tokio/time/

use crate::error::Result;
use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Utility functions for working with timestamps
pub mod time {
    use super::*;

    /// Get current Unix timestamp
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Convert days ago to Unix timestamp
    pub fn days_ago(days: u64) -> u64 {
        current_timestamp().saturating_sub(days * 24 * 60 * 60)
    }

    /// Convert hours ago to Unix timestamp
    pub fn hours_ago(hours: u64) -> u64 {
        current_timestamp().saturating_sub(hours * 60 * 60)
    }

    /// Convert minutes ago to Unix timestamp
    pub fn minutes_ago(minutes: u64) -> u64 {
        current_timestamp().saturating_sub(minutes * 60)
    }

    /// Format Unix timestamp to human-readable string
    pub fn format_timestamp(timestamp: u64) -> String {
        // Simple formatting without external dependencies
        let seconds_since = current_timestamp().saturating_sub(timestamp);

        if seconds_since < 60 {
            format!("{} seconds ago", seconds_since)
        } else if seconds_since < 3600 {
            format!("{} minutes ago", seconds_since / 60)
        } else if seconds_since < 86400 {
            format!("{} hours ago", seconds_since / 3600)
        } else {
            format!("{} days ago", seconds_since / 86400)
        }
    }

    /// Get start of day timestamp for a given number of days ago
    pub fn start_of_day_ago(days: u64) -> u64 {
        let timestamp = days_ago(days);
        // Round down to start of day (midnight UTC)
        timestamp - (timestamp % 86400)
    }
}

/// Utility functions for working with Ethereum addresses
pub mod address {
    /// Check if a string is a valid Ethereum address format
    pub fn is_valid_ethereum_address(address: &str) -> bool {
        address.len() == 42
            && address.starts_with("0x")
            && address[2..].chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Normalize an Ethereum address to lowercase
    pub fn normalize_address(address: &str) -> String {
        if is_valid_ethereum_address(address) {
            address.to_lowercase()
        } else {
            address.to_string()
        }
    }

    /// Format address for display (show first and last few characters)
    pub fn format_address(address: &str, prefix_len: usize, suffix_len: usize) -> String {
        if address.len() <= prefix_len + suffix_len + 3 {
            address.to_string()
        } else {
            format!(
                "{}...{}",
                &address[0..prefix_len],
                &address[address.len() - suffix_len..]
            )
        }
    }

    /// Format address with default display format (6 prefix, 4 suffix)
    pub fn format_address_default(address: &str) -> String {
        format_address(address, 6, 4)
    }
}

/// Utility functions for working with financial data
pub mod finance {
    /// Format USD amount with appropriate precision
    pub fn format_usd(amount: f64) -> String {
        if amount >= 1_000_000_000.0 {
            format!("${:.1}B", amount / 1_000_000_000.0)
        } else if amount >= 1_000_000.0 {
            format!("${:.1}M", amount / 1_000_000.0)
        } else if amount >= 1_000.0 {
            format!("${:.1}K", amount / 1_000.0)
        } else if amount >= 1.0 {
            format!("${:.2}", amount)
        } else {
            format!("${:.4}", amount)
        }
    }

    /// Format percentage with appropriate precision
    pub fn format_percentage(percentage: f64) -> String {
        let abs_percentage = percentage.abs();
        if percentage.fract() == 0.0 {
            // Whole number, no decimals
            format!("{:.0}%", percentage)
        } else if abs_percentage >= 100.0 {
            format!("{:.0}%", percentage)
        } else if abs_percentage >= 10.0 {
            format!("{:.1}%", percentage)
        } else {
            format!("{:.2}%", percentage)
        }
    }

    /// Calculate percentage change between two values
    pub fn percentage_change(old_value: f64, new_value: f64) -> Option<f64> {
        if old_value == 0.0 {
            None
        } else {
            Some(((new_value - old_value) / old_value) * 100.0)
        }
    }

    /// Check if a value is considered "high" relative to a threshold
    pub fn is_high_value(value: f64, threshold: f64) -> bool {
        value >= threshold
    }

    /// Calculate compound annual growth rate (CAGR)
    pub fn calculate_cagr(initial_value: f64, final_value: f64, years: f64) -> Option<f64> {
        if initial_value <= 0.0 || final_value <= 0.0 || years <= 0.0 {
            None
        } else {
            Some(((final_value / initial_value).powf(1.0 / years) - 1.0) * 100.0)
        }
    }
}

/// Retry utilities for handling transient errors
pub mod retry {
    use super::*;
    use tokio::time::sleep;

    /// Retry configuration
    #[derive(Debug, Clone)]
    pub struct RetryConfig {
        /// Maximum number of retry attempts
        pub max_attempts: u32,
        /// Base delay between retries
        pub base_delay: Duration,
        /// Multiplier for exponential backoff
        pub backoff_multiplier: f64,
        /// Maximum delay between retries
        pub max_delay: Duration,
    }

    impl Default for RetryConfig {
        fn default() -> Self {
            Self {
                max_attempts: 3,
                base_delay: Duration::from_millis(500),
                backoff_multiplier: 2.0,
                max_delay: Duration::from_secs(30),
            }
        }
    }

    /// Retry a future with exponential backoff
    pub async fn retry_with_backoff<T, F, Fut>(config: RetryConfig, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut attempt = 0;
        let mut delay = config.base_delay;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= config.max_attempts || !e.is_retryable() => {
                    return Err(e);
                }
                Err(_) => {
                    // Wait before retrying
                    sleep(delay).await;

                    // Exponential backoff
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * config.backoff_multiplier) as u64,
                        ),
                        config.max_delay,
                    );
                }
            }
        }
    }

    /// Simple retry without backoff
    pub async fn retry_simple<T, F, Fut>(max_attempts: u32, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let config = RetryConfig {
            max_attempts,
            base_delay: Duration::from_millis(100),
            backoff_multiplier: 1.0,
            max_delay: Duration::from_millis(100),
        };
        retry_with_backoff(config, operation).await
    }
}

/// Pagination utilities
pub mod pagination {
    use crate::models::PaginatedResponse;
    use std::marker::PhantomData;

    /// Iterator for paginated API results
    pub struct PaginationIterator<T> {
        current_offset: u32,
        page_size: u32,
        total_items: Option<u64>,
        has_more: bool,
        _phantom: PhantomData<T>, // Fixed: Use PhantomData to handle unused generic parameter
    }

    impl<T> PaginationIterator<T> {
        /// Create a new pagination iterator
        pub fn new(page_size: u32) -> Self {
            Self {
                current_offset: 0,
                page_size,
                total_items: None,
                has_more: true,
                _phantom: PhantomData,
            }
        }

        /// Update iterator state with response data
        pub fn update_from_response(&mut self, response: &PaginatedResponse<T>) {
            self.total_items = response.total;
            self.has_more = response.has_more.unwrap_or(false);
            self.current_offset += response.data.len() as u32;
        }

        /// Check if there are more pages available
        pub fn has_next_page(&self) -> bool {
            self.has_more
        }

        /// Get the next page offset
        pub fn next_offset(&self) -> u32 {
            self.current_offset
        }

        /// Get the page size
        pub fn page_size(&self) -> u32 {
            self.page_size
        }

        /// Get total items if available
        pub fn total_items(&self) -> Option<u64> {
            self.total_items
        }

        /// Calculate progress percentage
        pub fn progress_percentage(&self) -> Option<f64> {
            self.total_items.map(|total| {
                if total == 0 {
                    100.0
                } else {
                    (self.current_offset as f64 / total as f64) * 100.0
                }
            })
        }
    }
}

/// Rate limiting utilities
pub mod rate_limit {
    use std::collections::VecDeque;
    use std::time::{Duration, Instant};

    /// Simple rate limiter using token bucket algorithm
    pub struct RateLimiter {
        max_requests: usize,
        window_duration: Duration,
        requests: VecDeque<Instant>,
    }

    impl RateLimiter {
        /// Create a new rate limiter
        pub fn new(max_requests: usize, window_duration: Duration) -> Self {
            Self {
                max_requests,
                window_duration,
                requests: VecDeque::new(),
            }
        }

        /// Check if a request can be made
        pub fn can_request(&mut self) -> bool {
            self.cleanup_old_requests();
            self.requests.len() < self.max_requests
        }

        /// Record a request (call this after making a request)
        pub fn record_request(&mut self) {
            self.cleanup_old_requests();
            self.requests.push_back(Instant::now());
        }

        /// Get time until next request is allowed
        pub fn time_until_next_request(&mut self) -> Duration {
            self.cleanup_old_requests();

            if self.requests.len() < self.max_requests {
                Duration::ZERO
            } else if let Some(oldest) = self.requests.front() {
                let elapsed = oldest.elapsed();
                self.window_duration.saturating_sub(elapsed)
            } else {
                Duration::ZERO
            }
        }

        /// Remove requests outside the current window
        fn cleanup_old_requests(&mut self) {
            let cutoff = Instant::now() - self.window_duration;
            while let Some(front) = self.requests.front() {
                if *front < cutoff {
                    self.requests.pop_front();
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_utils() {
        let now = time::current_timestamp();
        let day_ago = time::days_ago(1);
        let hour_ago = time::hours_ago(1);
        let minute_ago = time::minutes_ago(1);

        assert!(now > day_ago);
        assert!(now > hour_ago);
        assert!(now > minute_ago);
        assert!(day_ago < hour_ago);
        assert!(hour_ago < minute_ago);
    }

    #[test]
    fn test_address_validation() {
        assert!(address::is_valid_ethereum_address(
            "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82"
        ));
        assert!(!address::is_valid_ethereum_address(
            "742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82"
        ));
        assert!(!address::is_valid_ethereum_address(
            "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b8Z"
        ));
        assert!(!address::is_valid_ethereum_address("0x123")); // Too short
    }

    #[test]
    fn test_address_formatting() {
        let addr = "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82";
        let formatted = address::format_address(addr, 6, 4);
        assert_eq!(formatted, "0x742d...3b82");

        let default_formatted = address::format_address_default(addr);
        assert_eq!(default_formatted, "0x742d...3b82");
    }

    #[test]
    fn test_usd_formatting() {
        assert_eq!(finance::format_usd(1_500_000_000.0), "$1.5B");
        assert_eq!(finance::format_usd(2_500_000.0), "$2.5M");
        assert_eq!(finance::format_usd(1_500.0), "$1.5K");
        assert_eq!(finance::format_usd(15.50), "$15.50");
        assert_eq!(finance::format_usd(0.1234), "$0.1234");
    }

    #[test]
    fn test_percentage_formatting() {
        assert_eq!(finance::format_percentage(150.0), "150%");
        assert_eq!(finance::format_percentage(15.5), "15.5%");
        assert_eq!(finance::format_percentage(1.55), "1.55%");
    }

    #[test]
    fn test_percentage_change() {
        assert_eq!(finance::percentage_change(100.0, 110.0), Some(10.0));
        assert_eq!(finance::percentage_change(100.0, 90.0), Some(-10.0));
        assert_eq!(finance::percentage_change(0.0, 100.0), None);
    }

    #[test]
    fn test_cagr_calculation() {
        // 100 to 200 over 2 years should be ~41.42% CAGR
        let cagr = finance::calculate_cagr(100.0, 200.0, 2.0).unwrap();
        assert!((cagr - 41.42).abs() < 0.1);

        // Invalid inputs should return None
        assert_eq!(finance::calculate_cagr(0.0, 100.0, 1.0), None);
        assert_eq!(finance::calculate_cagr(100.0, 0.0, 1.0), None);
        assert_eq!(finance::calculate_cagr(100.0, 200.0, 0.0), None);
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = rate_limit::RateLimiter::new(2, Duration::from_secs(1));

        // Should allow first two requests
        assert!(limiter.can_request());
        limiter.record_request();
        assert!(limiter.can_request());
        limiter.record_request();

        // Should block third request
        assert!(!limiter.can_request());

        // Should have some time until next request
        let wait_time = limiter.time_until_next_request();
        assert!(wait_time > Duration::ZERO);
    }

    #[test]
    fn test_pagination_iterator() {
        let paginator = pagination::PaginationIterator::<String>::new(10);

        assert_eq!(paginator.page_size(), 10);
        assert_eq!(paginator.next_offset(), 0);
        assert!(paginator.has_next_page());
        assert_eq!(paginator.total_items(), None);
    }
}
````
