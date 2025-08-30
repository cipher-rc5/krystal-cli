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
