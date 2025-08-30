#!/usr/bin/env nu

# --- Ensure API key is set ---
if not ("KRYSTAL_API_KEY" in $env) or ($env.KRYSTAL_API_KEY | str trim | is-empty) {
    print "API key not found or empty"
    let key = (input "Enter your KRYSTAL_API_KEY: ")
    if ($key | str trim | is-empty) {
        error make {msg: "API key cannot be empty"}
    }
    $env.KRYSTAL_API_KEY = $key
}

# --- User variables (customize as needed) ---
let chain_id = 1
let protocol = "uniswapv3"
let sort_by = 0
let limit = 2

# --- API request ---
http get --headers {KC-APIKey: $env.KRYSTAL_API_KEY Content-Type: "application/json"} "https://cloud-api.krystal.app/v1/pools?chainId=($chain_id)&protocol=($protocol)&sortBy=($sort_by)&limit=($limit)"
| to json
