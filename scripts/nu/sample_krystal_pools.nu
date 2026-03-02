#!/usr/bin/env nu

# --- 10 objects from Krystal Pools ---

http get --headers {KC-APIKey: $env.KRYSTAL_API_KEY Content-Type: "application/json"} "https://cloud-api.krystal.app/v1/pools?chainId=1&protocol=uniswapv3&sortBy=0&limit=10"
| select chain.name chain.id chain.explorer poolAddress poolPrice protocol.key protocol.factoryAddress feeTier token0.address token0.symbol token0.decimals token1.address token1.symbol token1.decimals tvl stats1h.volume stats1h.fee stats1h.apr stats24h.volume stats24h.fee stats24h.apr stats7d.volume stats7d.fee stats7d.apr stats30d.volume stats30d.fee stats30d.apr
| to json






# # Define user variables
# let chain_id = 1
# let protocol = "uniswapv3"
# let sort_by = 0
# let limit = 2

# # Use them in the query string with proper interpolation
# http get --headers {
#     KC-APIKey: $env.KRYSTAL_API_KEY
#     Content-Type: "application/json"
# } $"https://cloud-api.krystal.app/v1/pools?chainId=($chain_id)&protocol=($protocol)&sortBy=($sort_by)&limit=($limit)"
