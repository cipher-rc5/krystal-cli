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
