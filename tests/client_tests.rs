//! Tests for the API client functionality

use krystal_cli::client::{KrystalApiClient};

#[test]
fn test_client_creation() {
    let client = KrystalApiClient::new("test-key".to_string());
    assert!(client.is_ok());
}
