// file: src/cli/commands.rs
// description:
// docs_reference:

use crate::cli::app::PositionStatusArg;
use crate::cli::app::OutputFormat;
use crate::cli::app::Commands;
use crate::cli::output::*;
use crate::error::Result;
use crate::query::*;
use crate::utils::time;
use crate::KrystalApiClient;

/// Execute a CLI command
pub async fn execute_command(command: Commands, client: &KrystalApiClient, format: OutputFormat) -> Result<()> {
    match command {
        Commands::Chains { detailed, chain_id, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_chains(client, detailed, chain_id, effective_format).await
        }
        Commands::Pools {
            chain_id,
            limit,
            protocol,
            token,
            factory,
            sort_by,
            min_tvl,
            min_volume,
            with_incentives,
            detailed,
            offset,
            format: cmd_format,
        } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_pools(
                client,
                chain_id,
                limit,
                protocol,
                token,
                factory,
                sort_by,
                min_tvl,
                min_volume,
                with_incentives,
                detailed,
                offset,
                effective_format,
            )
            .await
        }
        Commands::PoolDetail {
            chain_id,
            pool_address,
            factory,
            with_incentives,
        } => {
            handle_pool_detail(client, chain_id, &pool_address, factory.as_deref(), with_incentives, &format)
                .await
        }
        Commands::PoolHistory {
            chain_id,
            pool_address,
            factory,
            start_time,
            end_time,
            days_ago,
        } => {
            handle_pool_history(
                client,
                chain_id,
                &pool_address,
                factory.as_deref(),
                start_time,
                end_time,
                days_ago,
                &format,
            )
            .await
        }
        Commands::PoolTransactions {
            chain_id,
            pool_address,
            factory,
            start_time,
            end_time,
            days_ago,
            limit,
            offset,
        } => {
            handle_pool_transactions(
                client,
                chain_id,
                &pool_address,
                factory.as_deref(),
                start_time,
                end_time,
                days_ago,
                limit,
                offset,
                &format,
            )
            .await
        }
        Commands::Positions {
            wallet,
            chain_id,
            status,
            protocols,
            detailed,
            format: cmd_format,
        } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_positions(client, &wallet, chain_id, status, protocols, detailed, effective_format)
                .await
        }
        Commands::PositionDetail {
            chain_id,
            position_id,
        } => handle_position_detail(client, chain_id, &position_id, &format).await,
        Commands::PositionTransactions {
            chain_id,
            wallet,
            token_address,
            token_id,
            start_time,
            end_time,
            days_ago,
            limit,
        } => {
            handle_position_transactions(
                client,
                chain_id,
                wallet.as_deref(),
                &token_address,
                token_id.as_deref(),
                start_time,
                end_time,
                days_ago,
                limit,
                &format,
            )
            .await
        }
        Commands::Protocols { detailed, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_protocols(client, detailed, effective_format).await
        }
        Commands::ChainStats { chain_id, format: cmd_format } => {
            let effective_format = cmd_format.as_ref().unwrap_or(&format);
            handle_chain_stats(client, chain_id, effective_format).await
        }
    }
}

async fn handle_chains(
    client: &KrystalApiClient,
    detailed: bool,
    chain_id: Option<u32>,
    format: &OutputFormat,
) -> Result<()> {
    let chains = client.get_chains().await?;

    let filtered_chains: Vec<_> = if let Some(id) = chain_id {
        chains.into_iter().filter(|c| c.id == id).collect()
    } else {
        chains
    };

    match format {
        OutputFormat::Json => print_json(&filtered_chains)?,
        OutputFormat::Csv => print_chains_csv(&filtered_chains, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_chains_table(&filtered_chains, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_pools(
    client: &KrystalApiClient,
    chain_id: Option<u32>,
    limit: u32,
    protocol: Option<String>,
    token: Option<String>,
    factory: Option<String>,
    sort_by: Option<crate::cli::app::PoolSortBy>,
    min_tvl: Option<u32>,
    min_volume: Option<u32>,
    with_incentives: bool,
    detailed: bool,
    offset: u32,
    format: &OutputFormat,
) -> Result<()> {
    let mut query = PoolsQuery::new().limit(limit).offset(offset);

    if let Some(cid) = chain_id {
        query = query.chain_id(cid);
    }
    if let Some(proto) = protocol {
        query = query.protocol(proto);
    }
    if let Some(token_addr) = token {
        query = query.token(token_addr);
    }
    if let Some(factory_addr) = factory {
        query = query.factory_address(factory_addr);
    }
    if let Some(sort) = sort_by {
        query = query.sort_by(sort.into());
    }
    if let Some(tvl) = min_tvl {
        query = query.min_tvl(tvl);
    }
    if let Some(volume) = min_volume {
        query = query.min_volume_24h(volume);
    }
    if with_incentives {
        query = query.with_incentives(true);
    }

    let pools = client.get_pools(query).await?;

    match format {
        OutputFormat::Json => print_json(&pools)?,
        OutputFormat::Csv => print_pools_csv(&pools, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_pools_table(&pools, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_pool_detail(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    with_incentives: bool,
    format: &OutputFormat,
) -> Result<()> {
    let pool = client
        .get_pool_detail(chain_id, pool_address, factory_address, with_incentives)
        .await?;

    match format {
        OutputFormat::Json => print_json(&pool)?,
        _ => print_pool_detail(&pool)?,
    }

    Ok(())
}

async fn handle_pool_history(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, None, None);

    let history = client
        .get_pool_historical(chain_id, pool_address, factory_address, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&history)?,
        _ => {
            println!("Historical data for pool {}:", pool_address);
            print_json(&history)?;
        }
    }

    Ok(())
}

async fn handle_pool_transactions(
    client: &KrystalApiClient,
    chain_id: u32,
    pool_address: &str,
    factory_address: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: u32,
    offset: u32,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, Some(limit), Some(offset));

    let transactions = client
        .get_pool_transactions(chain_id, pool_address, factory_address, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&transactions)?,
        OutputFormat::Csv => print_transactions_csv(&transactions)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_transactions_table(&transactions, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_positions(
    client: &KrystalApiClient,
    wallet: &str,
    chain_id: Option<u32>,
    status: Option<PositionStatusArg>,
    protocols: Vec<String>,
    detailed: bool,
    format: &OutputFormat,
) -> Result<()> {
    let mut query = PositionsQuery::new(wallet);

    if let Some(cid) = chain_id {
        query = query.chain_id(cid);
    }
    if let Some(status_arg) = status {
        query = query.status(status_arg.into());
    }
    if !protocols.is_empty() {
        query = query.protocols(protocols);
    }

    let positions = client.get_positions(query).await?;

    match format {
        OutputFormat::Json => print_json(&positions)?,
        OutputFormat::Csv => print_positions_csv(&positions, detailed)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_positions_table(&positions, detailed, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_position_detail(
    client: &KrystalApiClient,
    chain_id: u32,
    position_id: &str,
    format: &OutputFormat,
) -> Result<()> {
    let position = client.get_position_detail(chain_id, position_id).await?;

    match format {
        OutputFormat::Json => print_json(&position)?,
        _ => print_position_detail(&position)?,
    }

    Ok(())
}

async fn handle_position_transactions(
    client: &KrystalApiClient,
    chain_id: u32,
    wallet: Option<&str>,
    token_address: &str,
    token_id: Option<&str>,
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: u32,
    format: &OutputFormat,
) -> Result<()> {
    let query = build_transaction_query(start_time, end_time, days_ago, Some(limit), None);

    let transactions = client
        .get_position_transactions(chain_id, wallet, token_address, token_id, query)
        .await?;

    match format {
        OutputFormat::Json => print_json(&transactions)?,
        OutputFormat::Csv => print_transactions_csv(&transactions)?,
        OutputFormat::Table | OutputFormat::Compact => {
            print_transactions_table(&transactions, matches!(format, OutputFormat::Compact))?
        }
    }

    Ok(())
}

async fn handle_protocols(
    client: &KrystalApiClient,
    detailed: bool,
    format: &OutputFormat,
) -> Result<()> {
    let protocols = client.get_protocols().await?;

    match format {
        OutputFormat::Json => print_json(&protocols)?,
        _ => {
            println!("Supported Protocols:");
            if detailed {
                print_json(&protocols)?;
            } else {
                // Try to extract protocol names from the response
                if let Some(protocols_array) = protocols.as_array() {
                    for (i, protocol) in protocols_array.iter().enumerate() {
                        if let Some(name) = protocol.get("name").and_then(|n| n.as_str()) {
                            println!("{}. {}", i + 1, name);
                        } else if let Some(key) = protocol.get("key").and_then(|k| k.as_str()) {
                            println!("{}. {}", i + 1, key);
                        }
                    }
                } else {
                    print_json(&protocols)?;
                }
            }
        }
    }

    Ok(())
}

async fn handle_chain_stats(
    client: &KrystalApiClient,
    chain_id: u32,
    format: &OutputFormat,
) -> Result<()> {
    let stats = client.get_chain_stats(chain_id).await?;

    match format {
        OutputFormat::Json => print_json(&stats)?,
        _ => {
            println!("Chain {} Statistics:", chain_id);
            print_json(&stats)?;
        }
    }

    Ok(())
}

/// Helper function to build transaction query from various time parameters
fn build_transaction_query(
    start_time: Option<u64>,
    end_time: Option<u64>,
    days_ago: Option<u64>,
    limit: Option<u32>,
    offset: Option<u32>,
) -> Option<TransactionQuery> {
    let mut query = TransactionQuery::new();
    let mut has_params = false;

    if let Some(days) = days_ago {
        query = query.start_time(time::days_ago(days));
        has_params = true;
    } else if let Some(start) = start_time {
        query = query.start_time(start);
        has_params = true;
    }

    if let Some(end) = end_time {
        query = query.end_time(end);
        has_params = true;
    }

    if let Some(lmt) = limit {
        query = query.limit(lmt);
        has_params = true;
    }

    if let Some(off) = offset {
        query = query.offset(off);
        has_params = true;
    }

    if has_params {
        Some(query)
    } else {
        None
    }
}
