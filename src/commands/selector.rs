//! `selector` — compute the 4-byte selector of a function signature.
//!
//! The selector is the first 4 bytes of `keccak256` of the *canonical*
//! signature (no spaces, no parameter names, no return types). We parse the
//! signature with alloy first so the canonical form — and the hashing — are
//! handled correctly regardless of incidental whitespace.

use alloy::json_abi::Function;
use alloy::primitives::hex;

use crate::errors;

/// Entry point: parse `signature` and return its selector as `0x`-prefixed hex.
pub fn run(signature: &str) -> errors::Result<String> {
    let func = Function::parse(signature)?;
    Ok(format!("0x{}", hex::encode(func.selector())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_erc20_transfer_selector() {
        assert_eq!(run("transfer(address,uint256)").unwrap(), "0xa9059cbb");
    }

    #[test]
    fn tolerates_whitespace_in_signature() {
        assert_eq!(run("transfer(address, uint256)").unwrap(), "0xa9059cbb");
    }

    #[test]
    fn errors_on_garbage_signature() {
        assert!(run("not a signature").is_err());
    }
}
