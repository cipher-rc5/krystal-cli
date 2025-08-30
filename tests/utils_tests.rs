use krystal_cli::utils::{address, finance, time, retry, pagination, rate_limit};
use std::time::Duration;

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
fn test_time_formatting() {
    let now = time::current_timestamp();
    let recent = now - 30; // 30 seconds ago
    let formatted = time::format_timestamp(recent);
    assert!(formatted.contains("seconds ago"));

    let hour_ago = now - 3700; // About an hour ago
    let formatted = time::format_timestamp(hour_ago);
    assert!(formatted.contains("hour"));
}

#[test]
fn test_start_of_day() {
    let start_today = time::start_of_day_ago(0);
    let start_yesterday = time::start_of_day_ago(1);

    // Should be exactly 24 hours apart
    assert_eq!(start_today - start_yesterday, 86400);

    // Should be divisible by 86400 (start of day)
    assert_eq!(start_today % 86400, 0);
    assert_eq!(start_yesterday % 86400, 0);
}

#[test]
fn test_address_validation() {
    assert!(address::is_valid_ethereum_address(
        "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82"
    ));
    assert!(address::is_valid_ethereum_address(
        "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48" // Mixed case
    ));

    // Invalid cases
    assert!(!address::is_valid_ethereum_address(
        "742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82" // Missing 0x
    ));
    assert!(!address::is_valid_ethereum_address(
        "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b8Z" // Invalid hex character
    ));
    assert!(!address::is_valid_ethereum_address("0x123")); // Too short
    assert!(!address::is_valid_ethereum_address(
        "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b821" // Too long
    ));
}

#[test]
fn test_address_normalization() {
    let addr = "0x742D35CC6639C0532FA20C00FA1A5A6F1A8F3B82";
    let normalized = address::normalize_address(addr);
    assert_eq!(normalized, "0x742d35cc6639c0532fa20c00fa1a5a6f1a8f3b82");

    let invalid_addr = "invalid";
    let normalized_invalid = address::normalize_address(invalid_addr);
    assert_eq!(normalized_invalid, "invalid"); // Should return as-is for invalid addresses
}

#[test]
fn test_address_formatting() {
    let addr = "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82";
    let formatted = address::format_address(addr, 6, 4);
    assert_eq!(formatted, "0x742d...3b82");

    let default_formatted = address::format_address_default(addr);
    assert_eq!(default_formatted, "0x742d...3b82");

    // Test with short address
    let short_addr = "0x123456";
    let formatted_short = address::format_address(short_addr, 6, 4);
    assert_eq!(formatted_short, "0x123456"); // Should return as-is
}

#[test]
fn test_usd_formatting() {
    assert_eq!(finance::format_usd(1_500_000_000.0), "$1.5B");
    assert_eq!(finance::format_usd(2_500_000.0), "$2.5M");
    assert_eq!(finance::format_usd(1_500.0), "$1.5K");
    assert_eq!(finance::format_usd(15.50), "$15.50");
    assert_eq!(finance::format_usd(0.1234), "$0.1234");
    assert_eq!(finance::format_usd(0.0), "$0.0000");
}

#[test]
fn test_percentage_formatting() {
    assert_eq!(finance::format_percentage(150.0), "150%");
    assert_eq!(finance::format_percentage(15.5), "15.5%");
    assert_eq!(finance::format_percentage(1.55), "1.55%");
    assert_eq!(finance::format_percentage(0.1), "0.10%");
    assert_eq!(finance::format_percentage(-5.0), "-5%");
}

#[test]
fn test_percentage_change() {
    assert_eq!(finance::percentage_change(100.0, 110.0), Some(10.0));
    assert_eq!(finance::percentage_change(100.0, 90.0), Some(-10.0));
    assert_eq!(finance::percentage_change(0.0, 100.0), None);
    assert_eq!(finance::percentage_change(50.0, 75.0), Some(50.0));
}

#[test]
fn test_high_value_detection() {
    assert!(finance::is_high_value(1000.0, 500.0));
    assert!(!finance::is_high_value(400.0, 500.0));
    assert!(finance::is_high_value(500.0, 500.0)); // Equal should be true
}

#[test]
fn test_cagr_calculation() {
    // 100 to 200 over 2 years should be ~41.42% CAGR
    let cagr = finance::calculate_cagr(100.0, 200.0, 2.0).unwrap();
    assert!((cagr - 41.42).abs() < 0.1);

    // 100 to 150 over 1 year should be 50% CAGR
    let cagr = finance::calculate_cagr(100.0, 150.0, 1.0).unwrap();
    assert!((cagr - 50.0).abs() < 0.1);

    // Invalid inputs should return None
    assert_eq!(finance::calculate_cagr(0.0, 100.0, 1.0), None);
    assert_eq!(finance::calculate_cagr(100.0, 0.0, 1.0), None);
    assert_eq!(finance::calculate_cagr(100.0, 200.0, 0.0), None);
    assert_eq!(finance::calculate_cagr(-100.0, 200.0, 1.0), None);
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
    assert!(wait_time <= Duration::from_secs(1));
}

#[test]
fn test_rate_limiter_cleanup() {
    let mut limiter = rate_limit::RateLimiter::new(1, Duration::from_millis(100));

    // Make a request
    assert!(limiter.can_request());
    limiter.record_request();
    assert!(!limiter.can_request());

    // Wait for window to pass
    std::thread::sleep(Duration::from_millis(150));

    // Should be able to make another request
    assert!(limiter.can_request());
}

#[test]
fn test_pagination_iterator() {
    let mut paginator = pagination::PaginationIterator::<String>::new(10);

    assert_eq!(paginator.page_size(), 10);
    assert_eq!(paginator.next_offset(), 0);
    assert!(paginator.has_next_page());
    assert_eq!(paginator.total_items(), None);
    assert_eq!(paginator.progress_percentage(), None);

    // Simulate updating with response data
    let mock_response = krystal_cli::models::PaginatedResponse {
        data: vec!["item1".to_string(), "item2".to_string()],
        total: Some(100),
        offset: Some(0),
        limit: Some(10),
        has_more: Some(true),
    };

    paginator.update_from_response(&mock_response);

    assert_eq!(paginator.total_items(), Some(100));
    assert_eq!(paginator.next_offset(), 2); // Should be updated with data length
    assert!(paginator.has_next_page());

    let progress = paginator.progress_percentage().unwrap();
    assert!((progress - 2.0).abs() < 0.1); // 2/100 = 2%
}

#[test]
fn test_pagination_iterator_no_more_pages() {
    let mut paginator = pagination::PaginationIterator::<String>::new(10);

    let mock_response = krystal_cli::models::PaginatedResponse {
        data: vec!["item1".to_string()],
        total: Some(1),
        offset: Some(0),
        limit: Some(10),
        has_more: Some(false),
    };

    paginator.update_from_response(&mock_response);

    assert!(!paginator.has_next_page());
    assert_eq!(paginator.progress_percentage().unwrap(), 100.0);
}

// Async tests for retry functionality
#[tokio::test]
async fn test_retry_success_on_first_attempt() {
    let result = retry::retry_simple(3, || async {
        Ok::<i32, krystal_cli::error::KrystalApiError>(42)
    }).await;

    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_retry_with_backoff_config() {
    let config = retry::RetryConfig {
        max_attempts: 2,
        base_delay: Duration::from_millis(10),
        backoff_multiplier: 2.0,
        max_delay: Duration::from_millis(100),
    };

    let attempt_count = std::cell::RefCell::new(0);
    let result = retry::retry_with_backoff(config, || {
        let current_attempt = {
            let mut count = attempt_count.borrow_mut();
            *count += 1;
            *count
        };
        async move {
            if current_attempt == 1 {
                Err(krystal_cli::error::KrystalApiError::ApiError {
                    status: 500,
                    message: "Internal Server Error".to_string(),
                })
            } else {
                Ok(42)
            }
        }
    }).await;

    assert_eq!(result.unwrap(), 42);
    assert_eq!(*attempt_count.borrow(), 2);
}
