//! `balance` — query an account's ETH balance from an EVM JSON-RPC endpoint.
//!
//! Parses the address, connects to the given RPC over HTTP, fetches the latest
//! balance (in wei), and returns it formatted as ether.

use std::str::FromStr;

use alloy::primitives::{Address, utils::format_ether};
use alloy::providers::{Provider, ProviderBuilder};

use crate::errors;

/// Entry point: fetch `address`'s balance from `rpc_url` and return it in ether.
pub async fn run(address: &str, rpc_url: &str) -> errors::Result<String> {
    // Validate the address (a bad address fails here, before any network call).
    let parsed = Address::from_str(address)?;

    // Connect to the RPC over HTTP and fetch the latest balance, in wei.
    let rpc_url = rpc_url.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);
    let balance = provider.get_balance(parsed).await?;

    // Render wei as a human-readable ether amount.
    Ok(format_ether(balance))
}
