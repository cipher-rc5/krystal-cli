// file: src/cli/app.rs
// description:
// docs_reference:

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
