// file: tests/live_api_smoke.rs
// description: Feature-gated smoke tests against the live Krystal Cloud API
// docs_reference: https://docs.rs/tokio/latest/tokio/

#![cfg(feature = "live-api-tests")]

use krystal_cli::{KrystalApiClient, PoolsQuery};

fn should_run_live_tests() -> bool {
    std::env::var("KRYSTAL_RUN_LIVE_TESTS")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn live_client() -> Option<KrystalApiClient> {
    if !should_run_live_tests() {
        return None;
    }

    KrystalApiClient::from_env().ok()
}

#[tokio::test]
async fn smoke_get_chains() {
    let Some(client) = live_client() else {
        return;
    };

    let chains = client.get_chains().await.unwrap();
    assert!(!chains.is_empty());
}

#[tokio::test]
async fn smoke_get_protocols() {
    let Some(client) = live_client() else {
        return;
    };

    let protocols = client.get_protocols().await.unwrap();
    assert!(!protocols.is_empty());
}

#[tokio::test]
async fn smoke_get_small_pool_page() {
    let Some(client) = live_client() else {
        return;
    };

    let pools = client.get_pools(PoolsQuery::new().limit(1)).await.unwrap();
    assert!(pools.len() <= 1);
}
