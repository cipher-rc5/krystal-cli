// file: src/utils.rs
// description: Utility functions and helpers for the Krystal API client, including time handling,
//             address validation, financial formatting, and retry logic for robust API interactions
// docs_reference: https://docs.rs/tokio/latest/tokio/time/

use crate::error::Result;
use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Utility functions for working with timestamps
pub mod time {
    use super::*;

    /// Get current Unix timestamp
    pub fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Convert days ago to Unix timestamp
    pub fn days_ago(days: u64) -> u64 {
        current_timestamp().saturating_sub(days * 24 * 60 * 60)
    }

    /// Convert hours ago to Unix timestamp
    pub fn hours_ago(hours: u64) -> u64 {
        current_timestamp().saturating_sub(hours * 60 * 60)
    }

    /// Convert minutes ago to Unix timestamp
    pub fn minutes_ago(minutes: u64) -> u64 {
        current_timestamp().saturating_sub(minutes * 60)
    }

    /// Format Unix timestamp to human-readable string
    pub fn format_timestamp(timestamp: u64) -> String {
        // Simple formatting without external dependencies
        let seconds_since = current_timestamp().saturating_sub(timestamp);

        if seconds_since < 60 {
            format!("{} seconds ago", seconds_since)
        } else if seconds_since < 3600 {
            format!("{} minutes ago", seconds_since / 60)
        } else if seconds_since < 86400 {
            format!("{} hours ago", seconds_since / 3600)
        } else {
            format!("{} days ago", seconds_since / 86400)
        }
    }

    /// Get start of day timestamp for a given number of days ago
    pub fn start_of_day_ago(days: u64) -> u64 {
        let timestamp = days_ago(days);
        // Round down to start of day (midnight UTC)
        timestamp - (timestamp % 86400)
    }
}

/// Utility functions for working with Ethereum addresses
pub mod address {
    /// Check if a string is a valid Ethereum address format (hex chars + length only)
    pub fn is_valid_ethereum_address(address: &str) -> bool {
        address.len() == 42
            && address.starts_with("0x")
            && address[2..].chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Validate an Ethereum address including EIP-55 mixed-case checksum if applicable.
    ///
    /// Accepts:
    /// - All-lowercase addresses (no checksum to validate)
    /// - All-uppercase addresses (no checksum to validate)
    /// - Mixed-case addresses that match their EIP-55 checksum
    pub fn is_valid_ethereum_address_checksum(address: &str) -> bool {
        if !is_valid_ethereum_address(address) {
            return false;
        }

        let hex = &address[2..];

        // All-lowercase or all-uppercase: no checksum encoding, accept as-is
        if hex.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()) {
            return true;
        }
        if hex.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()) {
            return true;
        }

        // Mixed-case: validate EIP-55 checksum using keccak of the lowercase hex
        let expected = eip55_checksum(hex);
        hex == expected
    }

    /// Compute the EIP-55 checksummed form of a 40-char hex string (without 0x prefix)
    fn eip55_checksum(hex_lower: &str) -> String {
        let lower = hex_lower.to_ascii_lowercase();
        let hash = keccak_hex(&lower);

        lower
            .chars()
            .zip(hash.chars())
            .map(|(c, h)| {
                let hash_nibble = h.to_digit(16).unwrap_or(0);
                if c.is_ascii_alphabetic() && hash_nibble >= 8 {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect()
    }

    /// Minimal Keccak-256 implementation for EIP-55 — returns hex digest.
    /// Uses the Keccak sponge (not SHA3-256; Ethereum uses the pre-NIST variant).
    fn keccak_hex(input: &str) -> String {
        // Rate = 1088 bits = 136 bytes, capacity = 512 bits, output = 256 bits
        const RATE: usize = 136;
        const OUTPUT_BYTES: usize = 32;

        let bytes = input.as_bytes();
        let mut state = [0u64; 25];

        // Absorb
        let mut buf = bytes.to_vec();
        // Keccak padding: 0x01 ... 0x80
        buf.push(0x01);
        while !buf.len().is_multiple_of(RATE) {
            buf.push(0x00);
        }
        *buf.last_mut().unwrap() ^= 0x80;

        for chunk in buf.chunks(RATE) {
            for (i, lane_bytes) in chunk.chunks(8).enumerate() {
                let mut lane = 0u64;
                for (j, &b) in lane_bytes.iter().enumerate() {
                    lane |= (b as u64) << (j * 8);
                }
                state[i] ^= lane;
            }
            keccak_f(&mut state);
        }

        // Squeeze
        let mut out = Vec::with_capacity(OUTPUT_BYTES);
        'outer: for lane in &state {
            for j in 0..8 {
                out.push(((lane >> (j * 8)) & 0xff) as u8);
                if out.len() == OUTPUT_BYTES {
                    break 'outer;
                }
            }
        }

        out.iter().map(|b| format!("{:02x}", b)).collect()
    }

    fn keccak_f(state: &mut [u64; 25]) {
        const RC: [u64; 24] = [
            0x0000000000000001, 0x0000000000008082, 0x800000000000808A, 0x8000000080008000,
            0x000000000000808B, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
            0x000000000000008A, 0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
            0x000000008000808B, 0x800000000000008B, 0x8000000000008089, 0x8000000000008003,
            0x8000000000008002, 0x8000000000000080, 0x000000000000800A, 0x800000008000000A,
            0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008,
        ];
        const RHO: [u32; 24] = [
            1, 62, 28, 27, 36, 44, 6, 55, 20, 3, 10, 43,
            25, 39, 41, 45, 15, 21, 8, 18, 2, 61, 56, 14,
        ];
        const PI: [usize; 24] = [
            10, 7, 11, 17, 18, 3, 5, 16, 8, 21, 24, 4,
            15, 23, 19, 13, 12, 2, 20, 14, 22, 9, 6, 1,
        ];

        for &rc in &RC {
            // Theta
            let mut c = [0u64; 5];
            for x in 0..5 {
                c[x] = state[x] ^ state[x + 5] ^ state[x + 10] ^ state[x + 15] ^ state[x + 20];
            }
            let mut d = [0u64; 5];
            for x in 0..5 {
                d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
            }
            for x in 0..5 {
                for y in 0..5 {
                    state[x + y * 5] ^= d[x];
                }
            }

            // Rho + Pi
            let mut b = [0u64; 25];
            b[0] = state[0];
            let mut current = state[1];
            for i in 0..24 {
                let next = state[PI[i]];
                b[PI[i]] = current.rotate_left(RHO[i]);
                current = next;
            }

            // Chi
            for y in 0..5 {
                let row = [
                    b[y * 5], b[1 + y * 5], b[2 + y * 5],
                    b[3 + y * 5], b[4 + y * 5],
                ];
                for x in 0..5 {
                    state[x + y * 5] = row[x] ^ ((!row[(x + 1) % 5]) & row[(x + 2) % 5]);
                }
            }

            // Iota
            state[0] ^= rc;
        }
    }

    /// Normalize an Ethereum address to lowercase
    pub fn normalize_address(address: &str) -> String {
        if is_valid_ethereum_address(address) {
            address.to_lowercase()
        } else {
            address.to_string()
        }
    }

    /// Return the EIP-55 checksummed form of an address
    pub fn to_checksum_address(address: &str) -> Option<String> {
        if !is_valid_ethereum_address(address) {
            return None;
        }
        let hex = &address[2..];
        Some(format!("0x{}", eip55_checksum(hex)))
    }

    /// Format address for display (show first and last few characters)
    pub fn format_address(address: &str, prefix_len: usize, suffix_len: usize) -> String {
        if address.len() <= prefix_len + suffix_len + 3 {
            address.to_string()
        } else {
            format!(
                "{}...{}",
                &address[0..prefix_len],
                &address[address.len() - suffix_len..]
            )
        }
    }

    /// Format address with default display format (6 prefix, 4 suffix)
    pub fn format_address_default(address: &str) -> String {
        format_address(address, 6, 4)
    }
}

/// Utility functions for working with financial data
pub mod finance {
    /// Format USD amount with appropriate precision
    pub fn format_usd(amount: f64) -> String {
        if amount >= 1_000_000_000.0 {
            format!("${:.1}B", amount / 1_000_000_000.0)
        } else if amount >= 1_000_000.0 {
            format!("${:.1}M", amount / 1_000_000.0)
        } else if amount >= 1_000.0 {
            format!("${:.1}K", amount / 1_000.0)
        } else if amount >= 1.0 {
            format!("${:.2}", amount)
        } else {
            format!("${:.4}", amount)
        }
    }

    /// Format percentage with appropriate precision
    pub fn format_percentage(percentage: f64) -> String {
        let abs_percentage = percentage.abs();
        if percentage.fract() == 0.0 {
            // Whole number, no decimals
            format!("{:.0}%", percentage)
        } else if abs_percentage >= 100.0 {
            format!("{:.0}%", percentage)
        } else if abs_percentage >= 10.0 {
            format!("{:.1}%", percentage)
        } else {
            format!("{:.2}%", percentage)
        }
    }

    /// Calculate percentage change between two values
    pub fn percentage_change(old_value: f64, new_value: f64) -> Option<f64> {
        if old_value == 0.0 {
            None
        } else {
            Some(((new_value - old_value) / old_value) * 100.0)
        }
    }

    /// Check if a value is considered "high" relative to a threshold
    pub fn is_high_value(value: f64, threshold: f64) -> bool {
        value >= threshold
    }

    /// Calculate compound annual growth rate (CAGR)
    pub fn calculate_cagr(initial_value: f64, final_value: f64, years: f64) -> Option<f64> {
        if initial_value <= 0.0 || final_value <= 0.0 || years <= 0.0 {
            None
        } else {
            Some(((final_value / initial_value).powf(1.0 / years) - 1.0) * 100.0)
        }
    }
}

/// Retry utilities for handling transient errors
pub mod retry {
    use super::*;
    use tokio::time::sleep;

    /// Retry configuration
    #[derive(Debug, Clone)]
    pub struct RetryConfig {
        /// Maximum number of retry attempts
        pub max_attempts: u32,
        /// Base delay between retries
        pub base_delay: Duration,
        /// Multiplier for exponential backoff
        pub backoff_multiplier: f64,
        /// Maximum delay between retries
        pub max_delay: Duration,
    }

    impl Default for RetryConfig {
        fn default() -> Self {
            Self {
                max_attempts: 3,
                base_delay: Duration::from_millis(500),
                backoff_multiplier: 2.0,
                max_delay: Duration::from_secs(30),
            }
        }
    }

    /// Retry a future with exponential backoff
    pub async fn retry_with_backoff<T, F, Fut>(config: RetryConfig, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let mut attempt = 0;
        let mut delay = config.base_delay;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) if attempt >= config.max_attempts || !e.is_retryable() => {
                    return Err(e);
                }
                Err(_) => {
                    // Wait before retrying
                    sleep(delay).await;

                    // Exponential backoff
                    delay = std::cmp::min(
                        Duration::from_millis(
                            (delay.as_millis() as f64 * config.backoff_multiplier) as u64,
                        ),
                        config.max_delay,
                    );
                }
            }
        }
    }

    /// Simple retry without backoff
    pub async fn retry_simple<T, F, Fut>(max_attempts: u32, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let config = RetryConfig {
            max_attempts,
            base_delay: Duration::from_millis(100),
            backoff_multiplier: 1.0,
            max_delay: Duration::from_millis(100),
        };
        retry_with_backoff(config, operation).await
    }
}

/// Pagination utilities
pub mod pagination {
    use crate::models::PaginatedResponse;
    use std::marker::PhantomData;

    /// Iterator for paginated API results
    pub struct PaginationIterator<T> {
        current_offset: u32,
        page_size: u32,
        total_items: Option<u64>,
        has_more: bool,
        _phantom: PhantomData<T>, // Fixed: Use PhantomData to handle unused generic parameter
    }

    impl<T> PaginationIterator<T> {
        /// Create a new pagination iterator
        pub fn new(page_size: u32) -> Self {
            Self {
                current_offset: 0,
                page_size,
                total_items: None,
                has_more: true,
                _phantom: PhantomData,
            }
        }

        /// Update iterator state with response data
        pub fn update_from_response(&mut self, response: &PaginatedResponse<T>) {
            self.total_items = response.total;
            self.has_more = response.has_more.unwrap_or(false);
            self.current_offset += response.data.len() as u32;
        }

        /// Check if there are more pages available
        pub fn has_next_page(&self) -> bool {
            self.has_more
        }

        /// Get the next page offset
        pub fn next_offset(&self) -> u32 {
            self.current_offset
        }

        /// Get the page size
        pub fn page_size(&self) -> u32 {
            self.page_size
        }

        /// Get total items if available
        pub fn total_items(&self) -> Option<u64> {
            self.total_items
        }

        /// Calculate progress percentage
        pub fn progress_percentage(&self) -> Option<f64> {
            self.total_items.map(|total| {
                if total == 0 {
                    100.0
                } else {
                    (self.current_offset as f64 / total as f64) * 100.0
                }
            })
        }
    }
}

/// Rate limiting utilities
pub mod rate_limit {
    use std::collections::VecDeque;
    use std::time::{Duration, Instant};

    /// Simple rate limiter using token bucket algorithm
    pub struct RateLimiter {
        max_requests: usize,
        window_duration: Duration,
        requests: VecDeque<Instant>,
    }

    impl RateLimiter {
        /// Create a new rate limiter
        pub fn new(max_requests: usize, window_duration: Duration) -> Self {
            Self {
                max_requests,
                window_duration,
                requests: VecDeque::new(),
            }
        }

        /// Check if a request can be made
        pub fn can_request(&mut self) -> bool {
            self.cleanup_old_requests();
            self.requests.len() < self.max_requests
        }

        /// Record a request (call this after making a request)
        pub fn record_request(&mut self) {
            self.cleanup_old_requests();
            self.requests.push_back(Instant::now());
        }

        /// Get time until next request is allowed
        pub fn time_until_next_request(&mut self) -> Duration {
            self.cleanup_old_requests();

            if self.requests.len() < self.max_requests {
                Duration::ZERO
            } else if let Some(oldest) = self.requests.front() {
                let elapsed = oldest.elapsed();
                self.window_duration.saturating_sub(elapsed)
            } else {
                Duration::ZERO
            }
        }

        /// Remove requests outside the current window
        fn cleanup_old_requests(&mut self) {
            let cutoff = Instant::now() - self.window_duration;
            while let Some(front) = self.requests.front() {
                if *front < cutoff {
                    self.requests.pop_front();
                } else {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_address_validation() {
        assert!(address::is_valid_ethereum_address(
            "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82"
        ));
        assert!(!address::is_valid_ethereum_address(
            "742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82"
        ));
        assert!(!address::is_valid_ethereum_address(
            "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b8Z"
        ));
        assert!(!address::is_valid_ethereum_address("0x123")); // Too short
    }

    #[test]
    fn test_address_checksum_validation() {
        // All-lowercase is always accepted
        assert!(address::is_valid_ethereum_address_checksum(
            "0x742d35cc6639c0532fa20c00fa1a5a6f1a8f3b82"
        ));
        // All-uppercase is always accepted
        assert!(address::is_valid_ethereum_address_checksum(
            "0x742D35CC6639C0532FA20C00FA1A5A6F1A8F3B82"
        ));
        // Invalid format rejected
        assert!(!address::is_valid_ethereum_address_checksum("0x123"));
        assert!(!address::is_valid_ethereum_address_checksum("not-an-address"));
    }

    #[test]
    fn test_checksum_address_generation() {
        let lower = "0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed";
        let checksummed = address::to_checksum_address(lower);
        assert!(checksummed.is_some());
        let result = checksummed.unwrap();
        assert!(result.starts_with("0x"));
        assert_eq!(result.len(), 42);
        // Round-trip: the result should pass checksum validation
        assert!(address::is_valid_ethereum_address_checksum(&result));
    }

    #[test]
    fn test_address_formatting() {
        let addr = "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82";
        let formatted = address::format_address(addr, 6, 4);
        assert_eq!(formatted, "0x742d...3b82");

        let default_formatted = address::format_address_default(addr);
        assert_eq!(default_formatted, "0x742d...3b82");
    }

    #[test]
    fn test_usd_formatting() {
        assert_eq!(finance::format_usd(1_500_000_000.0), "$1.5B");
        assert_eq!(finance::format_usd(2_500_000.0), "$2.5M");
        assert_eq!(finance::format_usd(1_500.0), "$1.5K");
        assert_eq!(finance::format_usd(15.50), "$15.50");
        assert_eq!(finance::format_usd(0.1234), "$0.1234");
    }

    #[test]
    fn test_percentage_formatting() {
        assert_eq!(finance::format_percentage(150.0), "150%");
        assert_eq!(finance::format_percentage(15.5), "15.5%");
        assert_eq!(finance::format_percentage(1.55), "1.55%");
    }

    #[test]
    fn test_percentage_change() {
        assert_eq!(finance::percentage_change(100.0, 110.0), Some(10.0));
        assert_eq!(finance::percentage_change(100.0, 90.0), Some(-10.0));
        assert_eq!(finance::percentage_change(0.0, 100.0), None);
    }

    #[test]
    fn test_cagr_calculation() {
        // 100 to 200 over 2 years should be ~41.42% CAGR
        let cagr = finance::calculate_cagr(100.0, 200.0, 2.0).unwrap();
        assert!((cagr - 41.42).abs() < 0.1);

        // Invalid inputs should return None
        assert_eq!(finance::calculate_cagr(0.0, 100.0, 1.0), None);
        assert_eq!(finance::calculate_cagr(100.0, 0.0, 1.0), None);
        assert_eq!(finance::calculate_cagr(100.0, 200.0, 0.0), None);
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
    }

    #[test]
    fn test_pagination_iterator() {
        let paginator = pagination::PaginationIterator::<String>::new(10);

        assert_eq!(paginator.page_size(), 10);
        assert_eq!(paginator.next_offset(), 0);
        assert!(paginator.has_next_page());
        assert_eq!(paginator.total_items(), None);
    }
}
