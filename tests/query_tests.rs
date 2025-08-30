//! Tests for query builders

use krystal_cli::query::*;
use krystal_cli::models::{PoolSortBy, PositionStatus};

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

    let invalid_query_high_limit = PoolsQuery::new().limit(5000);
    assert!(invalid_query_high_limit.validate().is_err());

    let valid_query = PoolsQuery::new().limit(100);
    assert!(valid_query.validate().is_ok());

    let invalid_tvl_query = PoolsQuery::new().min_tvl(2_000_000_000);
    assert!(invalid_tvl_query.validate().is_err());
}

#[test]
fn test_pools_query_all_fields() {
    let query = PoolsQuery::new()
        .chain_id(137)
        .factory_address("0x1234567890123456789012345678901234567890")
        .protocol("sushiswap")
        .token("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd")
        .sort_by(PoolSortBy::Volume24h)
        .min_tvl(50000)
        .min_volume_24h(100000)
        .limit(25)
        .offset(50)
        .with_incentives(true);

    assert_eq!(query.chain_id, Some(137));
    assert_eq!(query.factory_address, Some("0x1234567890123456789012345678901234567890".to_string()));
    assert_eq!(query.protocol, Some("sushiswap".to_string()));
    assert_eq!(query.token, Some("0xabcdefabcdefabcdefabcdefabcdefabcdefabcd".to_string()));
    assert_eq!(query.sort_by, Some(PoolSortBy::Volume24h));
    assert_eq!(query.min_tvl, Some(50000));
    assert_eq!(query.min_volume_24h, Some(100000));
    assert_eq!(query.limit, Some(25));
    assert_eq!(query.offset, Some(50));
    assert_eq!(query.with_incentives, Some(true));

    assert!(query.validate().is_ok());
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
fn test_positions_query_protocols_builder() {
    let protocols = vec!["Uniswap V3", "SushiSwap", "Curve"];
    let query = PositionsQuery::new("0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82")
        .protocols(protocols.clone());

    assert_eq!(
        query.protocols,
        Some(protocols.into_iter().map(|s| s.to_string()).collect())
    );
}

#[test]
fn test_positions_query_validation() {
    let invalid_query = PositionsQuery::new("invalid-address");
    assert!(invalid_query.validate().is_err());

    let invalid_query_no_0x = PositionsQuery::new("742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82");
    assert!(invalid_query_no_0x.validate().is_err());

    let invalid_query_too_short = PositionsQuery::new("0x123");
    assert!(invalid_query_too_short.validate().is_err());

    let valid_query = PositionsQuery::new("0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82");
    assert!(valid_query.validate().is_ok());

    let empty_wallet_query = PositionsQuery::new("");
    assert!(empty_wallet_query.validate().is_err());
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

    let invalid_limit_zero = TransactionQuery::new().limit(0);
    assert!(invalid_limit_zero.validate().is_err());

    let invalid_limit_high = TransactionQuery::new().limit(50000);
    assert!(invalid_limit_high.validate().is_err());

    let valid_query = TransactionQuery::new().time_range(1000000, 2000000);
    assert!(valid_query.validate().is_ok());

    let valid_query_separate_times = TransactionQuery::new()
        .start_time(1000000)
        .end_time(2000000);
    assert!(valid_query_separate_times.validate().is_ok());
}

#[test]
fn test_query_builder_chaining() {
    let query = PoolsQuery::new()
        .chain_id(1)
        .protocol("uniswapv3")
        .limit(50)
        .sort_by(PoolSortBy::Apr)
        .with_incentives(true);

    // Test that all fields are set correctly
    assert_eq!(query.chain_id, Some(1));
    assert_eq!(query.protocol, Some("uniswapv3".to_string()));
    assert_eq!(query.limit, Some(50));
    assert_eq!(query.sort_by, Some(PoolSortBy::Apr));
    assert_eq!(query.with_incentives, Some(true));
}

#[test]
fn test_default_query_builders() {
    let pools_query = PoolsQuery::new();
    assert_eq!(pools_query.chain_id, None);
    assert_eq!(pools_query.limit, None);
    assert!(pools_query.validate().is_ok());

    let tx_query = TransactionQuery::new();
    assert_eq!(tx_query.start_time, None);
    assert_eq!(tx_query.end_time, None);
    assert!(tx_query.validate().is_ok());
}
