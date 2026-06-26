//! `selector` — compute the 4-byte selector of a function signature.
//!
//! The selector is the first 4 bytes of `keccak256` of the *canonical*
//! signature (no spaces, no parameter names, no return types). We parse the
//! signature with alloy first so the canonical form — and the hashing — are
//! handled correctly regardless of incidental whitespace.

use alloy_json_abi::Function;
use alloy_primitives::hex;

use crate::{commands::Command, errors};

pub struct Selector {
    signature: String,
}

impl Selector {
    pub fn new(signature: String) -> Self {
        Selector { signature }
    }
}

impl Command for Selector {
    fn run(&self) -> errors::Result<String> {
        let func = Function::parse(&self.signature)?;
        Ok(format!("0x{}", hex::encode(func.selector())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_erc20_transfer_selector() {
        let selector = Selector::new("transfer(address,uint256)".to_string());
        assert_eq!(selector.run().unwrap(), "0xa9059cbb");
    }

    #[test]
    fn tolerates_whitespace_in_signature() {
        let selector = Selector::new("transfer(address, uint256)".to_string());
        assert_eq!(selector.run().unwrap(), "0xa9059cbb");
    }

    #[test]
    fn errors_on_garbage_signature() {
        let selector = Selector::new("not a signature".to_string());
        assert!(selector.run().is_err());
    }
}
