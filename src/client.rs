// file: src/client.rs
// description: Main API client implementation for the Krystal Cloud API, handling HTTP requests,
//             authentication, response parsing, retry with exponential backoff, rate limiting,
//             and providing high-level methods for API interaction
// docs_reference: https://docs.rs/reqwest/latest/reqwest/

use crate::error::{KrystalApiError, Result};
use crate::models::*;
use crate::query::*;
use crate::utils::rate_limit::RateLimiter;
use crate::utils::retry::{RetryConfig, retry_with_backoff};
use log::debug;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::env;
use std::sync::Mutex;
use std::time::Duration;
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
    /// Retry configuration for transient failures
    pub retry: RetryConfig,
    /// Maximum requests per second (0 = unlimited)
    pub max_requests_per_second: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            base_url: "https://cloud-api.krystal.app".to_string(),
            timeout_secs: 30,
            user_agent: format!("krystal-rust-client/{}", env!("CARGO_PKG_VERSION")),
            retry: RetryConfig::default(),
            max_requests_per_second: 10,
        }
    }
}

/// Main API client for interacting with the Krystal Cloud API
pub struct KrystalApiClient {
    client: Client,
    config: ClientConfig,
    api_key: String,
    rate_limiter: Mutex<RateLimiter>,
}

impl std::fmt::Debug for KrystalApiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KrystalApiClient")
            .field("config", &self.config)
            .field("api_key", &"[redacted]")
            .finish()
    }
}

impl KrystalApiClient {
    /// Create a new API client with custom configuration
    pub fn with_config(api_key: String, config: ClientConfig) -> Result<Self> {
        let api_key = api_key.trim().to_string();
        if api_key.is_empty() {
            return Err(KrystalApiError::AuthError);
        }

        Url::parse(&config.base_url)?;

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .user_agent(&config.user_agent)
            .https_only(true)
            .build()?;

        let rate_limiter = RateLimiter::new(
            config.max_requests_per_second,
            Duration::from_secs(1),
        );

        Ok(Self {
            client,
            config,
            api_key,
            rate_limiter: Mutex::new(rate_limiter),
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
    async fn handle_response(response: Response) -> Result<Value> {
        let status = response.status();
        let url = response.url().to_string();

        debug!("Response: {} from {}", status.as_u16(), url);

        match status.as_u16() {
            200..=299 => response
                .json::<Value>()
                .await
                .map_err(KrystalApiError::from),
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

    fn endpoint(&self, path: &str) -> Result<Url> {
        let mut base = Url::parse(&self.config.base_url)?;
        if !base.path().ends_with('/') {
            let next_path = format!("{}/", base.path());
            base.set_path(&next_path);
        }

        Ok(base.join(path)?)
    }

    fn parse_item<T: DeserializeOwned>(json: Value, context: &str) -> Result<T> {
        serde_json::from_value(json).map_err(|e| {
            KrystalApiError::InvalidParams(format!("Failed to parse {context} response: {e}"))
        })
    }

    fn parse_items<T: DeserializeOwned>(items: &[Value], context: &str) -> Result<Vec<T>> {
        items
            .iter()
            .cloned()
            .map(|item| Self::parse_item(item, context))
            .collect()
    }

    /// Create a GET request with authentication headers
    fn authenticated_get(&self, url: Url) -> reqwest::RequestBuilder {
        self.client.get(url).header("KC-APIKey", &self.api_key)
    }

    /// Enforce rate limit, waiting if necessary
    async fn enforce_rate_limit(&self) {
        loop {
            let wait_duration = {
                let mut limiter = self.rate_limiter.lock().expect("rate limiter poisoned");
                if limiter.can_request() {
                    limiter.record_request();
                    break;
                }
                limiter.time_until_next_request()
            };
            if wait_duration.is_zero() {
                tokio::time::sleep(Duration::from_millis(1)).await;
            } else {
                debug!("Rate limit reached, waiting {}ms", wait_duration.as_millis());
                tokio::time::sleep(wait_duration).await;
            }
        }
    }

    /// Execute a GET request with rate limiting and retry
    async fn get_with_retry(&self, url: Url) -> Result<Value> {
        let config = self.config.retry.clone();
        retry_with_backoff(config, || async {
            self.enforce_rate_limit().await;
            let url = url.clone();
            debug!("GET {}", url);
            let response = self.authenticated_get(url).send().await?;
            Self::handle_response(response).await
        })
        .await
    }

    /// Get list of all supported blockchain networks
    pub async fn get_chains(&self) -> Result<Vec<ChainInfo>> {
        let url = self.endpoint("v1/chains")?;
        let json = self.get_with_retry(url).await?;

        let chains_data = json
            .as_array()
            .or_else(|| json.get("chains").and_then(|c| c.as_array()))
            .ok_or_else(|| {
                KrystalApiError::InvalidParams("Invalid chains response format".to_string())
            })?;

        Self::parse_items(chains_data, "chains")
    }

    /// Get stats for a specific chain
    pub async fn get_chain_stats(&self, chain_id: u32) -> Result<ChainStats> {
        let url = self.endpoint(&format!("v1/chains/{chain_id}"))?;
        let json = self.get_with_retry(url).await?;
        let payload = json.get("chain").cloned().unwrap_or(json);
        Self::parse_item(payload, "chain stats")
    }

    /// Get pool data with filtering options
    pub async fn get_pools(&self, query: PoolsQuery) -> Result<Vec<Pool>> {
        query.validate().map_err(KrystalApiError::InvalidParams)?;

        let mut url = self.endpoint("v1/pools")?;
        self.build_pools_query_params(&mut url, &query);

        let json = self.get_with_retry(url).await?;

        let pools_data = json
            .get("pools")
            .and_then(|p| p.as_array())
            .or_else(|| json.as_array())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        Self::parse_items(pools_data, "pools")
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
        if let Some(tvl_from) = query.min_tvl {
            query_pairs.append_pair("tvlFrom", &format!("{tvl_from}"));
        }
        if let Some(volume_from) = query.min_volume_24h {
            query_pairs.append_pair("volume24hFrom", &format!("{volume_from}"));
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
        let mut url = self.endpoint(&format!("v1/pools/{chain_id}/{pool_address}"))?;

        {
            let mut query_pairs = url.query_pairs_mut();
            if let Some(factory) = factory_address {
                query_pairs.append_pair("factoryAddress", factory);
            }
            query_pairs.append_pair("withIncentives", &with_incentives.to_string());
        }

        let json = self.get_with_retry(url).await?;
        Self::parse_item(json, "pool detail")
    }

    /// Get historical data for a specific pool
    pub async fn get_pool_historical(
        &self,
        chain_id: u32,
        pool_address: &str,
        factory_address: Option<&str>,
        query: Option<TransactionQuery>,
    ) -> Result<PoolHistoricalData> {
        let mut url = self.endpoint(&format!("v1/pools/{chain_id}/{pool_address}/historical"))?;

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

        let json = self.get_with_retry(url).await?;
        Self::parse_item(json, "pool historical")
    }

    /// Get transactions for a specific pool
    pub async fn get_pool_transactions(
        &self,
        chain_id: u32,
        pool_address: &str,
        factory_address: Option<&str>,
        query: Option<TransactionQuery>,
    ) -> Result<Vec<Transaction>> {
        if let Some(q) = &query {
            q.validate().map_err(KrystalApiError::InvalidParams)?;
        }

        let mut url = self.endpoint(&format!("v1/pools/{chain_id}/{pool_address}/transactions"))?;

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

        let json = self.get_with_retry(url).await?;

        let txs_data = json
            .get("transactions")
            .and_then(|t| t.as_array())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        Self::parse_items(txs_data, "pool transactions")
    }

    /// Get all positions for a wallet
    pub async fn get_positions(&self, query: PositionsQuery) -> Result<Vec<Position>> {
        query.validate().map_err(KrystalApiError::InvalidParams)?;

        let mut url = self.endpoint("v1/positions")?;

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("wallet", &query.wallet);

            if let Some(chain_id) = query.chain_id {
                query_pairs.append_pair("chainId", &chain_id.to_string());
            }
            if let Some(ref status) = query.position_status
                && let Some(status_str) = status.as_str()
            {
                query_pairs.append_pair("positionStatus", status_str);
            }
            if let Some(ref protocols) = query.protocols {
                for protocol in protocols {
                    query_pairs.append_pair("protocols", protocol);
                }
            }
        }

        let json = self.get_with_retry(url).await?;

        let positions_data = json
            .get("positions")
            .and_then(|p| p.as_array())
            .or_else(|| json.as_array())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        Self::parse_items(positions_data, "positions")
    }

    /// Get detailed information about a specific position
    pub async fn get_position_detail(&self, chain_id: u32, position_id: &str) -> Result<Position> {
        let url = self.endpoint(&format!("v1/positions/{chain_id}/{position_id}"))?;
        let json = self.get_with_retry(url).await?;
        Self::parse_item(json, "position detail")
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
            q.validate().map_err(KrystalApiError::InvalidParams)?;
        }

        let mut url = self.endpoint(&format!("v1/positions/{chain_id}/transactions"))?;

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

        let json = self.get_with_retry(url).await?;

        let txs_data = json
            .get("transactions")
            .and_then(|t| t.as_array())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        Self::parse_items(txs_data, "position transactions")
    }

    /// Get list of all supported protocols
    pub async fn get_protocols(&self) -> Result<Vec<ProtocolSummary>> {
        let url = self.endpoint("v1/protocols")?;
        let json = self.get_with_retry(url).await?;

        let protocols_data = json
            .as_array()
            .or_else(|| json.get("protocols").and_then(|p| p.as_array()))
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        Self::parse_items(protocols_data, "protocols")
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

    /// Get pools with full pagination metadata
    pub async fn get_pools_paginated(
        &self,
        query: PoolsQuery,
    ) -> Result<PaginatedResponse<Pool>> {
        query.validate().map_err(KrystalApiError::InvalidParams)?;

        let limit = query.limit;
        let offset = query.offset;

        let mut url = self.endpoint("v1/pools")?;
        self.build_pools_query_params(&mut url, &query);

        let json = self.get_with_retry(url).await?;

        let pools_data = json
            .get("pools")
            .and_then(|p| p.as_array())
            .or_else(|| json.as_array())
            .map(Vec::as_slice)
            .unwrap_or(&[]);

        let pools: Vec<Pool> = Self::parse_items(pools_data, "pools")?;

        let total = json
            .get("total")
            .and_then(|v| v.as_u64());
        let has_more = json
            .get("hasMore")
            .and_then(|v| v.as_bool())
            .or_else(|| total.map(|t| pools_data.len() as u64 + (offset.unwrap_or(0) as u64) < t));

        Ok(PaginatedResponse {
            data: pools,
            total,
            offset: offset.map(|o| o as u64),
            limit: limit.map(|l| l as u64),
            has_more,
        })
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
    fn test_client_rejects_empty_key() {
        let client = KrystalApiClient::new("".to_string());
        assert!(client.is_err());
        assert!(matches!(client.unwrap_err(), KrystalApiError::AuthError));
    }

    #[test]
    fn test_client_rejects_whitespace_only_key() {
        let client = KrystalApiClient::new("   ".to_string());
        assert!(client.is_err());
        assert!(matches!(client.unwrap_err(), KrystalApiError::AuthError));
    }

    #[test]
    fn test_client_trims_whitespace_from_key() {
        let client = KrystalApiClient::new("  test-key  ".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_with_custom_config() {
        let config = ClientConfig {
            base_url: "https://api.example.com".to_string(),
            timeout_secs: 60,
            user_agent: "test-client/1.0".to_string(),
            retry: RetryConfig::default(),
            max_requests_per_second: 5,
        };

        let client = KrystalApiClient::with_config("test-key".to_string(), config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_client_rejects_invalid_base_url() {
        let config = ClientConfig {
            base_url: "not-a-url".to_string(),
            ..ClientConfig::default()
        };
        let client = KrystalApiClient::with_config("test-key".to_string(), config);
        assert!(client.is_err());
    }

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.base_url, "https://cloud-api.krystal.app");
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.max_requests_per_second, 10);
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

    #[test]
    fn test_pools_query_params_all_fields() {
        let mut url = Url::parse("https://api.example.com/test").unwrap();
        let query = PoolsQuery::new()
            .chain_id(1)
            .protocol("uniswapv3")
            .min_tvl(100_000.0)
            .min_volume_24h(50_000.0)
            .limit(25)
            .offset(10)
            .with_incentives(true)
            .sort_by(PoolSortBy::Tvl);

        let client = KrystalApiClient::new("test".to_string()).unwrap();
        client.build_pools_query_params(&mut url, &query);

        let qs = url.query().unwrap_or("");
        assert!(qs.contains("chainId=1"));
        assert!(qs.contains("protocol=uniswapv3"));
        assert!(qs.contains("tvlFrom="));
        assert!(qs.contains("volume24hFrom="));
        assert!(qs.contains("limit=25"));
        assert!(qs.contains("offset=10"));
        assert!(qs.contains("withIncentives=true"));
        assert!(qs.contains("sortBy=1"));
    }

    #[test]
    fn test_endpoint_construction() {
        let client = KrystalApiClient::new("test-key".to_string()).unwrap();
        let url = client.endpoint("v1/chains").unwrap();
        assert_eq!(url.as_str(), "https://cloud-api.krystal.app/v1/chains");
    }

    #[test]
    fn test_debug_redacts_api_key() {
        let client = KrystalApiClient::new("super-secret-key".to_string()).unwrap();
        let debug_str = format!("{:?}", client);
        assert!(!debug_str.contains("super-secret-key"));
        assert!(debug_str.contains("[redacted]"));
    }
}
