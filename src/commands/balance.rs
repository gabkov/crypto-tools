use std::str::FromStr;

use alloy::primitives::Address;
use alloy::primitives::utils::format_ether;
use alloy::{providers::Provider, providers::ProviderBuilder};

use crate::errors;

pub async fn run(address: &str, rpc: &str) -> errors::Result<String> {
    let parsed = Address::from_str(address)?;

    let rpc_url = rpc.parse()?;
    let provider = ProviderBuilder::new().connect_http(rpc_url);

    let balance = provider.get_balance(parsed).await?;

    Ok(format_ether(balance))
}
