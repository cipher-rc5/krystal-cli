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
