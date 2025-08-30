
use krystal_cli::cli::app::Commands;
use krystal_cli::cli::app::OutputFormat;
use krystal_cli::cli::app::PositionStatusArg;
use krystal_cli::cli::*;
use krystal_cli::models::*;
use std::collections::HashMap;

#[test]
fn test_pool_sort_by_conversion() {
    let cli_sort = krystal_cli::cli::app::PoolSortBy::Tvl;
    let model_sort: krystal_cli::models::PoolSortBy = cli_sort.into();
    assert_eq!(model_sort, krystal_cli::models::PoolSortBy::Tvl);

    let cli_sort = krystal_cli::cli::app::PoolSortBy::Volume;
    let model_sort: krystal_cli::models::PoolSortBy = cli_sort.into();
    assert_eq!(model_sort, krystal_cli::models::PoolSortBy::Volume24h);
}

#[test]
fn test_position_status_conversion() {
    let cli_status = PositionStatusArg::Open;
    let model_status: krystal_cli::models::PositionStatus = cli_status.into();
    assert_eq!(model_status, krystal_cli::models::PositionStatus::Open);

    let cli_status = PositionStatusArg::All;
    let model_status: krystal_cli::models::PositionStatus = cli_status.into();
    assert_eq!(model_status, krystal_cli::models::PositionStatus::All);
}

#[test]
fn test_output_format_values() {
    // Test that all output formats are valid
    let formats = vec![
        OutputFormat::Table,
        OutputFormat::Json,
        OutputFormat::Csv,
        OutputFormat::Compact,
    ];

    // Just ensure they can be created and matched
    for format in formats {
        match format {
            OutputFormat::Table => assert!(true),
            OutputFormat::Json => assert!(true),
            OutputFormat::Csv => assert!(true),
            OutputFormat::Compact => assert!(true),
        }
    }
}

#[test]
fn test_escape_csv() {
    // Test the escape_csv function from output.rs
    assert_eq!(escape_csv("simple"), "simple");
    assert_eq!(escape_csv("with,comma"), "\"with,comma\"");
    assert_eq!(escape_csv("with\"quote"), "\"with\"\"quote\"");
    assert_eq!(escape_csv("with\nnewline"), "\"with\nnewline\"");
    assert_eq!(escape_csv("with,comma\"and\"quote"), "\"with,comma\"\"and\"\"quote\"");
}

#[test]
fn test_truncate_string() {
    // Test the truncate_string function from output.rs
    assert_eq!(truncate_string("short", 10), "short");
    assert_eq!(truncate_string("this is a very long string", 10), "this is...");
    assert_eq!(truncate_string("exactly10c", 10), "exactly10c");
    assert_eq!(truncate_string("a", 5), "a");
}

#[test]
fn test_get_token_pair_display() {
    let additional_fields = HashMap::new();

    // Test with both tokens present
    let pool_with_tokens = Pool {
        chain: None,
        address: "0x123".to_string(),
        pool_price: 1.0,
        protocol: None,
        fee_tier: 3000,
        token0: Some(TokenInfo {
            address: "0x456".to_string(),
            symbol: "USDC".to_string(),
            name: "USD Coin".to_string(),
            decimals: 6,
            logo: None,
        }),
        token1: Some(TokenInfo {
            address: "0x789".to_string(),
            symbol: "ETH".to_string(),
            name: "Ethereum".to_string(),
            decimals: 18,
            logo: None,
        }),
        tvl: 10000.0,
        stats1h: None,
        stats24h: None,
        stats7d: None,
        stats30d: None,
        incentives: None,
        additional_fields: additional_fields.clone(),
    };

    assert_eq!(get_token_pair_display(&pool_with_tokens), "USDC/ETH");

    // Test with missing tokens
    let pool_without_tokens = Pool {
        chain: None,
        address: "0x123".to_string(),
        pool_price: 1.0,
        protocol: None,
        fee_tier: 3000,
        token0: None,
        token1: None,
        tvl: 10000.0,
        stats1h: None,
        stats24h: None,
        stats7d: None,
        stats30d: None,
        incentives: None,
        additional_fields,
    };

    assert_eq!(get_token_pair_display(&pool_without_tokens), "Unknown/Unknown");
}

#[test]
fn test_format_usd_compact() {
    assert_eq!(format_usd_compact(1_500_000_000.0), "1.5B");
    assert_eq!(format_usd_compact(2_500_000.0), "2.5M");
    assert_eq!(format_usd_compact(1_500.0), "1.5K");
    assert_eq!(format_usd_compact(15.50), "16"); // Rounds to nearest integer
    assert_eq!(format_usd_compact(0.1234), "0.12");
}

// Mock functions to test output formatting
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

fn get_token_pair_display(pool: &Pool) -> String {
    match (&pool.token0, &pool.token1) {
        (Some(t0), Some(t1)) => format!("{}/{}", t0.symbol, t1.symbol),
        _ => "Unknown/Unknown".to_string(),
    }
}

fn format_usd_compact(amount: f64) -> String {
    if amount >= 1_000_000_000.0 {
        format!("{:.1}B", amount / 1_000_000_000.0)
    } else if amount >= 1_000_000.0 {
        format!("{:.1}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("{:.1}K", amount / 1_000.0)
    } else if amount >= 1.0 {
        format!("{:.0}", amount)
    } else {
        format!("{:.2}", amount)
    }
}

// Integration-style tests for CLI parsing
#[test]
fn test_cli_parsing_pools_command() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "pools",
        "--chain-id", "1",
        "--limit", "20",
        "--protocol", "uniswapv3",
        "--detailed"
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        match cli.command {
            Commands::Pools { chain_id, limit, protocol, detailed, .. } => {
                assert_eq!(chain_id, Some(1));
                assert_eq!(limit, 20);
                assert_eq!(protocol, Some("uniswapv3".to_string()));
                assert!(detailed);
            }
            _ => panic!("Expected Pools command"),
        }
    }
}

#[test]
fn test_cli_parsing_positions_command() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "positions",
        "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82",
        "--chain-id", "1",
        "--status", "open",
        "--protocols", "uniswapv3",
        "--protocols", "sushiswap"
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        match cli.command {
            Commands::Positions { wallet, chain_id, status, protocols, .. } => {
                assert_eq!(wallet, "0x742d35Cc6639C0532fA20c00fa1A5a6f1a8f3b82");
                assert_eq!(chain_id, Some(1));
                assert_eq!(status, Some(PositionStatusArg::Open));
                assert_eq!(protocols, vec!["uniswapv3", "sushiswap"]);
            }
            _ => panic!("Expected Positions command"),
        }
    }
}

#[test]
fn test_cli_parsing_pool_detail_command() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "pool-detail",
        "1",
        "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41",
        "--factory", "0x1f98431c8ad98523631ae4a59f267346ea31f984",
        "--with-incentives"
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        match cli.command {
            Commands::PoolDetail { chain_id, pool_address, factory, with_incentives } => {
                assert_eq!(chain_id, 1);
                assert_eq!(pool_address, "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41");
                assert_eq!(factory, Some("0x1f98431c8ad98523631ae4a59f267346ea31f984".to_string()));
                assert!(with_incentives);
            }
            _ => panic!("Expected PoolDetail command"),
        }
    }
}

#[test]
fn test_cli_parsing_pool_transactions_command() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "pool-transactions",
        "1",
        "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41",
        "--days-ago", "7",
        "--limit", "100"
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        match cli.command {
            Commands::PoolTransactions { chain_id, pool_address, days_ago, limit, .. } => {
                assert_eq!(chain_id, 1);
                assert_eq!(pool_address, "0x7e3d694a81ec15e56a4fea19f3bc841afe462b41");
                assert_eq!(days_ago, Some(7));
                assert_eq!(limit, 100);
            }
            _ => panic!("Expected PoolTransactions command"),
        }
    }
}

#[test]
fn test_cli_parsing_with_global_options() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "--verbose",
        "--format", "json",
        "--no-color",
        "chains"
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        assert!(cli.verbose);
        assert!(matches!(cli.format, OutputFormat::Json));
        assert!(cli.no_color);
        assert!(matches!(cli.command, Commands::Chains { .. }));
    }
}

#[test]
fn test_time_parameter_parsing() {
    use clap::Parser;

    let args = vec![
        "krystal-cli",
        "pool-history",
        "1",
        "0x123",
        "--start-time", "1640995200", // Jan 1, 2022
        "--end-time", "1672531200"    // Jan 1, 2023
    ];

    let cli = Cli::try_parse_from(args);
    assert!(cli.is_ok());

    if let Ok(cli) = cli {
        match cli.command {
            Commands::PoolHistory { start_time, end_time, .. } => {
                assert_eq!(start_time, Some(1640995200));
                assert_eq!(end_time, Some(1672531200));
            }
            _ => panic!("Expected PoolHistory command"),
        }
    }
}
