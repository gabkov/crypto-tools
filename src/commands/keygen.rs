//! `keygen` — generate a random private key and its Ethereum address.
//!
//! Key generation is secure (`random()` uses a cryptographic RNG). The risk is
//! *handling*: it prints the secret to stdout and keeps an un-zeroized copy.

use alloy_signer_local::PrivateKeySigner;

use crate::{commands::Command, errors};

pub struct Keygen {}

impl Keygen {
    pub fn new() -> Self {
        Keygen {}
    }

    fn render(signer: &PrivateKeySigner) -> String {
        format!(
            "private_key: 0x{}\naddress:     {}",
            alloy_primitives::hex::encode(signer.to_bytes()),
            signer.address(),
        )
    }
}

impl Command for Keygen {
    fn run(&self) -> errors::Result<String> {
        Ok(Keygen::render(&PrivateKeySigner::random()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn derives_known_address_from_private_key() {
        // Well-known test vector: private key = 1.
        let signer = PrivateKeySigner::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
        )
        .unwrap();
        let rendered = Keygen::render(&signer);
        assert!(rendered.contains(
            "private_key: 0x0000000000000000000000000000000000000000000000000000000000000001"
        ));
        assert!(rendered.contains("address:     0x7E5F4552091A69125d5DfCb7b8C2659029395Bdf"));
    }

    #[test]
    fn generates_distinct_keys() {
        let keygen1 = Keygen::new();
        let keygen2 = Keygen::new();
        // Two random generations should not collide.
        assert_ne!(keygen1.run().unwrap(), keygen2.run().unwrap());
    }
}
