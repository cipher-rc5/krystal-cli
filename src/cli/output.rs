// file: src/cli/output.rs
// description:
// docs_reference:

use crate::error::Result;
use crate::models::*;
use crate::utils::{address, finance};
use serde::Serialize;

/// Print data as JSON
pub fn print_json<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

/// Print chains in table format
pub fn print_chains_table(chains: &[ChainInfo], detailed: bool, compact: bool) -> Result<()> {
    if chains.is_empty() {
        println!("No chains found");
        return Ok(());
    }

    println!("Found {} supported chains", chains.len());

    if compact {
        for chain in chains {
            println!("{}: {}", chain.id, chain.name);
        }
    } else if detailed {
        for (i, chain) in chains.iter().enumerate() {
            println!("\n{}. {} (ID: {})", i + 1, chain.name, chain.id);
            if let Some(logo) = &chain.logo {
                println!("   Logo: {}", logo);
            }
            if let Some(explorer) = &chain.explorer {
                println!("   Explorer: {}", explorer);
            }
            if !chain.additional_fields.is_empty() {
                println!("   Additional fields: {:?}", chain.additional_fields);
            }
        }
    } else {
        println!("{:<4} {:<20} {:<50}", "ID", "Name", "Explorer");
        println!("{}", "-".repeat(75));

        for chain in chains {
            let explorer = chain.explorer.as_deref().unwrap_or("N/A");
            println!(
                "{:<4} {:<20} {:<50}",
                chain.id,
                truncate_string(&chain.name, 20),
                truncate_string(explorer, 50)
            );
        }
    }

    Ok(())
}

/// Print chains in CSV format
pub fn print_chains_csv(chains: &[ChainInfo], detailed: bool) -> Result<()> {
    if detailed {
        println!("id,name,logo,explorer");
        for chain in chains {
            println!(
                "{},{},{},{}",
                chain.id,
                escape_csv(&chain.name),
                chain.logo.as_deref().unwrap_or(""),
                chain.explorer.as_deref().unwrap_or("")
            );
        }
    } else {
        println!("id,name");
        for chain in chains {
            println!("{},{}", chain.id, escape_csv(&chain.name));
        }
    }
    Ok(())
}

/// Print pools in table format
pub fn print_pools_table(pools: &[Pool], detailed: bool, compact: bool) -> Result<()> {
    if pools.is_empty() {
        println!("No pools found");
        return Ok(());
    }

    println!("Found {} pools", pools.len());

    if compact {
        for pool in pools {
            let token_pair = get_token_pair_display(&pool);
            let protocol_name = pool.protocol.as_ref()
                .map(|p| p.name.as_str())
                .unwrap_or("Unknown");
            println!("{} ({}) - TVL: {}", token_pair, protocol_name, finance::format_usd(pool.tvl));
        }
    } else if detailed {
        for (i, pool) in pools.iter().enumerate() {
            print_pool_summary(i + 1, pool)?;
        }
    } else {
        print_pools_table_header();
        for (i, pool) in pools.iter().enumerate() {
            print_pool_table_row(i + 1, pool)?;
        }
    }

    Ok(())
}

/// Print pools in CSV format
pub fn print_pools_csv(pools: &[Pool], detailed: bool) -> Result<()> {
    if detailed {
        println!("index,chain_id,chain_name,pool_address,protocol,token0_symbol,token1_symbol,fee_tier,tvl,pool_price,volume_24h,apr_24h");
        for (i, pool) in pools.iter().enumerate() {
            let chain_info = pool.chain.as_ref();
            let token0_symbol = pool.token0.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
            let token1_symbol = pool.token1.as_ref().map_or("?".to_string(), |t| t.symbol.clone());
            let protocol_name = pool.protocol.as_ref().map_or("Unknown".to_string(), |p| p.name.clone());
            let volume_24h = pool.stats24h.as_ref().map(|s| s.volume).unwrap_or(0.0);
            let apr_24h = pool.stats24h.as_ref().map(|s| s.apr).unwrap_or(0.0);

            println!(
                "{},{},{},{},{},{},{},{},{},{},{},{}",
                i + 1,
                chain_info.map(|c| c.id).unwrap_or(0),
                escape_csv(&chain_info.map_or("Unknown".to_string(), |c| c.name.clone())),
                escape_csv(&pool.address),
                escape_csv(&protocol_name),
                escape_csv(&token0_symbol),
                escape_csv(&token1_symbol),
                pool.fee_tier,
                pool.tvl,
                pool.pool_price,
                volume_24h,
                apr_24h
            );
        }
    } else {
        println!("index,token_pair,protocol,tvl,volume_24h,apr_24h");
        for (i, pool) in pools.iter().enumerate() {
            let token_pair = get_token_pair_display(&pool);
            let protocol_name = pool.protocol.as_ref().map_or("Unknown".to_string(), |p| p.name.clone());
            let volume_24h = pool.stats24h.as_ref().map(|s| s.volume).unwrap_or(0.0);
            let apr_24h = pool.stats24h.as_ref().map(|s| s.apr).unwrap_or(0.0);

            println!(
                "{},{},{},{},{},{}",
                i + 1,
                escape_csv(&token_pair),
                escape_csv(&protocol_name),
                pool.tvl,
                volume_24h,
                apr_24h
            );
        }
    }
    Ok(())
}

/// Print detailed pool information
pub fn print_pool_detail(pool: &Pool) -> Result<()> {
    println!("\n{}", pool.display_name());
    println!("Address: {}", pool.address);

    if let Some(chain) = &pool.chain {
        println!("Chain: {} (ID: {})", chain.name, chain.id);
        if let Some(explorer) = &chain.explorer {
            println!("Explorer: {}", explorer);
        }
    }

    if let Some(protocol) = &pool.protocol {
        println!("Protocol: {} ({})", protocol.name, protocol.key);
        println!("Factory: {}", protocol.factory_address);
    }

    println!("Fee Tier: {}bps", pool.fee_tier);
    println!("TVL: {}", finance::format_usd(pool.tvl));
    println!("Pool Price: {:.8}", pool.pool_price);

    if let Some(token0) = &pool.token0 {
        println!("Token0: {} ({}) - {}", token0.symbol, token0.name, token0.address);
    }
    if let Some(token1) = &pool.token1 {
        println!("Token1: {} ({}) - {}", token1.symbol, token1.name, token1.address);
    }

    // Statistics
    if let Some(stats1h) = &pool.stats1h {
        println!("\n1h Statistics:");
        println!("  Volume: {}", finance::format_usd(stats1h.volume));
        println!("  Fees: {}", finance::format_usd(stats1h.fee));
        println!("  APR: {}", finance::format_percentage(stats1h.apr));
    }

    if let Some(stats24h) = &pool.stats24h {
        println!("\n24h Statistics:");
        println!("  Volume: {}", finance::format_usd(stats24h.volume));
        println!("  Fees: {}", finance::format_usd(stats24h.fee));
        println!("  APR: {}", finance::format_percentage(stats24h.apr));
    }

    if let Some(stats7d) = &pool.stats7d {
        println!("\n7d Statistics:");
        println!("  Volume: {}", finance::format_usd(stats7d.volume));
        println!("  Fees: {}", finance::format_usd(stats7d.fee));
        println!("  APR: {}", finance::format_percentage(stats7d.apr));
    }

    if let Some(stats30d) = &pool.stats30d {
        println!("\n30d Statistics:");
        println!("  Volume: {}", finance::format_usd(stats30d.volume));
        println!("  Fees: {}", finance::format_usd(stats30d.fee));
        println!("  APR: {}", finance::format_percentage(stats30d.apr));
    }

    // Incentives
    if let Some(incentives) = &pool.incentives {
        if !incentives.is_empty() {
            println!("\nIncentives:");
            for incentive in incentives {
                println!("  Type: {}", incentive.incentive_type);
                println!("  Token: {} ({})", incentive.token.symbol, incentive.token.name);
                println!("  Daily Reward: {}", finance::format_usd(incentive.daily_reward_usd));
                println!("  24h APR: {}", finance::format_percentage(incentive.apr24h));
                println!();
            }
        }
    }

    Ok(())
}

/// Print positions in table format
pub fn print_positions_table(positions: &[Position], detailed: bool, compact: bool) -> Result<()> {
    if positions.is_empty() {
        println!("No positions found");
        return Ok(());
    }

    println!("Found {} positions", positions.len());

    if compact {
        for position in positions {
            println!("{} - Status: {}, Value: {}",
                position.id,
                position.status,
                finance::format_usd(position.current_position_value)
            );
        }
    } else if detailed {
        for (i, pos) in positions.iter().enumerate() {
            print_position_summary(i + 1, pos)?;
        }
    } else {
        println!("{:<4} {:<20} {:<10} {:<12} {:<10} {:<8}",
            "#", "Position ID", "Status", "Value", "Chain", "Protocol");
        println!("{}", "-".repeat(70));

        for (i, pos) in positions.iter().enumerate() {
            let chain_name = pos.chain.as_ref()
                .map(|c| c.name.as_str())
                .unwrap_or("Unknown");
            let protocol_name = pos.pool.as_ref()
                .and_then(|p| p.protocol.as_ref())
                .map(|pr| pr.name.as_str())
                .unwrap_or("Unknown");

            println!(
                "{:<4} {:<20} {:<10} {:<12} {:<10} {:<8}",
                i + 1,
                truncate_string(&pos.id, 20),
                pos.status,
                finance::format_usd(pos.current_position_value),
                truncate_string(chain_name, 10),
                truncate_string(protocol_name, 8)
            );
        }
    }

    Ok(())
}

/// Print positions in CSV format
pub fn print_positions_csv(positions: &[Position], detailed: bool) -> Result<()> {
    if detailed {
        println!("index,position_id,chain_id,chain_name,status,current_value,min_price,max_price,liquidity");
        for (i, pos) in positions.iter().enumerate() {
            let chain_info = pos.chain.as_ref();
            println!(
                "{},{},{},{},{},{},{},{},{}",
                i + 1,
                escape_csv(&pos.id),
                chain_info.map(|c| c.id).unwrap_or(0),
                escape_csv(&chain_info.map(|c| &c.name).unwrap_or(&"Unknown".to_string())),
                escape_csv(&pos.status),
                pos.current_position_value,
                pos.min_price,
                pos.max_price,
                escape_csv(&pos.liquidity)
            );
        }
    } else {
        println!("index,position_id,status,current_value");
        for (i, pos) in positions.iter().enumerate() {
            println!(
                "{},{},{},{}",
                i + 1,
                escape_csv(&pos.id),
                escape_csv(&pos.status),
                pos.current_position_value
            );
        }
    }
    Ok(())
}

/// Print detailed position information
pub fn print_position_detail(position: &Position) -> Result<()> {
    println!("\nPosition: {}", position.id);
    println!("Owner: {}", address::format_address_default(&position.owner_address));
    println!("Token Address: {}", position.token_address);
    println!("Token ID: {}", position.token_id);
    println!("Status: {}", position.status);
    println!("Liquidity: {}", position.liquidity);
    println!("Price Range: {:.6} - {:.6}", position.min_price, position.max_price);
    println!("Current Value: {}", finance::format_usd(position.current_position_value));

    if let Some(chain) = &position.chain {
        println!("Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(pool) = &position.pool {
        println!("Pool: {}", pool.pool_address);
        if let Some(protocol) = &pool.protocol {
            println!("Protocol: {} ({})", protocol.name, protocol.key);
        }
    }

    if let Some(current_amounts) = &position.current_amounts {
        println!("\nCurrent Token Amounts:");
        for amount in current_amounts {
            println!("  {}: {} ({})",
                amount.token.symbol,
                amount.balance,
                finance::format_usd(amount.value)
            );
        }
    }

    if let Some(provided_amounts) = &position.provided_amounts {
        println!("\nProvided Token Amounts:");
        for amount in provided_amounts {
            println!("  {}: {} ({})",
                amount.token.symbol,
                amount.balance,
                finance::format_usd(amount.value)
            );
        }
    }

    if let Some(performance) = &position.performance {
        println!("\nPerformance:");
        println!("  Total Deposit Value: {}", finance::format_usd(performance.total_deposit_value));
        println!("  Total Withdraw Value: {}", finance::format_usd(performance.total_withdraw_value));
        println!("  P&L: {}", finance::format_usd(performance.pnl));
        println!("  ROI: {}", finance::format_percentage(performance.return_on_investment));
        println!("  Impermanent Loss: {}", finance::format_usd(performance.impermanent_loss));

        if let Some(compare_to_hold) = performance.compare_to_hold {
            println!("  Compare to Hold: {}", finance::format_percentage(compare_to_hold));
        }

        if let Some(apr) = &performance.apr {
            println!("  Total APR: {}", finance::format_percentage(apr.total_apr));
            println!("  Fee APR: {}", finance::format_percentage(apr.fee_apr));
            println!("  Farm APR: {}", finance::format_percentage(apr.farm_apr));
        }
    }

    Ok(())
}

/// Print transactions in table format
pub fn print_transactions_table(transactions: &[Transaction], compact: bool) -> Result<()> {
    if transactions.is_empty() {
        println!("No transactions found");
        return Ok(());
    }

    println!("Found {} transactions", transactions.len());

    if compact {
        for tx in transactions {
            println!("{}: {} - {:.4}/{:.4}",
                &tx.hash[0..10],
                tx.transaction_type,
                tx.amount0,
                tx.amount1
            );
        }
    } else {
        println!("{:<12} {:<10} {:<15} {:<15} {:<20}",
            "Hash", "Type", "Amount0", "Amount1", "Time");
        println!("{}", "-".repeat(75));

        for tx in transactions {
            let time_str = crate::utils::time::format_timestamp(tx.timestamp);
            println!(
                "{:<12} {:<10} {:<15.4} {:<15.4} {:<20}",
                &tx.hash[0..10],
                truncate_string(&tx.transaction_type, 10),
                tx.amount0,
                tx.amount1,
                truncate_string(&time_str, 20)
            );
        }
    }

    Ok(())
}

/// Print transactions in CSV format
pub fn print_transactions_csv(transactions: &[Transaction]) -> Result<()> {
    println!("hash,type,amount0,amount1,timestamp");
    for tx in transactions {
        println!(
            "{},{},{},{},{}",
            escape_csv(&tx.hash),
            escape_csv(&tx.transaction_type),
            tx.amount0,
            tx.amount1,
            tx.timestamp
        );
    }
    Ok(())
}

// Helper functions

fn print_pools_table_header() {
    println!("{:<4} {:<20} {:<15} {:<12} {:<12} {:<8}",
        "#", "Pool", "Protocol", "TVL", "24h Volume", "24h APR");
    println!("{}", "-".repeat(75));
}

fn print_pool_table_row(index: usize, pool: &Pool) -> Result<()> {
    let token_pair = get_token_pair_display(&pool);
    let protocol_name = pool.protocol.as_ref()
        .map(|p| p.key.as_str())
        .unwrap_or("Unknown");
    let volume_24h = pool.stats24h.as_ref()
        .map(|s| s.volume)
        .unwrap_or(0.0);
    let apr_24h = pool.stats24h.as_ref()
        .map(|s| s.apr)
        .unwrap_or(0.0);

    println!("{:<4} {:<20} {:<15} {:<12} {:<12} {:<8.1}%",
        index,
        truncate_string(&token_pair, 20),
        truncate_string(protocol_name, 15),
        format_usd_compact(pool.tvl),
        format_usd_compact(volume_24h),
        apr_24h
    );
    Ok(())
}

fn print_pool_summary(index: usize, pool: &Pool) -> Result<()> {
    println!("\n{}. {}", index, pool.display_name());
    println!("   Address: {}", address::format_address_default(&pool.address));

    if let Some(chain) = &pool.chain {
        println!("   Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(protocol) = &pool.protocol {
        println!("   Protocol: {} ({})", protocol.name, protocol.key);
    }

    println!("   Fee Tier: {}bps", pool.fee_tier);
    println!("   TVL: {}", finance::format_usd(pool.tvl));
    println!("   Pool Price: {:.8}", pool.pool_price);

    if let Some(stats24h) = &pool.stats24h {
        println!("   24h Volume: {}", finance::format_usd(stats24h.volume));
        println!("   24h Fees: {}", finance::format_usd(stats24h.fee));
        println!("   24h APR: {}", finance::format_percentage(stats24h.apr));
    }

    if let Some(stats7d) = &pool.stats7d {
        println!("   7d APR: {}", finance::format_percentage(stats7d.apr));
    }

    Ok(())
}

fn print_position_summary(index: usize, position: &Position) -> Result<()> {
    println!("\n{}. Position {}", index, position.id);
    println!("   Owner: {}", address::format_address_default(&position.owner_address));
    println!("   Status: {}", position.status);
    println!("   Value: {}", finance::format_usd(position.current_position_value));
    println!("   Price Range: {:.6} - {:.6}", position.min_price, position.max_price);

    if let Some(chain) = &position.chain {
        println!("   Chain: {} (ID: {})", chain.name, chain.id);
    }

    if let Some(pool) = &position.pool {
        if let Some(protocol) = &pool.protocol {
            println!("   Protocol: {}", protocol.name);
        }
    }

    Ok(())
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

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
