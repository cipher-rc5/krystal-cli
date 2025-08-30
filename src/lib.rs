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
