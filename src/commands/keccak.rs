//! `keccak` — compute the keccak-256 hash of some data.
//!
//! Hashes the UTF-8 bytes of the input by default; pass `--hex` to hash the
//! decoded bytes of a hex string instead.

use alloy_primitives::{hex, keccak256};

use crate::{commands::Command, errors};

pub struct Keccak {
    input: String,
    as_hex: bool,
}

impl Keccak {
    pub fn new(input: String, as_hex: bool) -> Self {
        Keccak { input, as_hex }
    }
}

impl Command for Keccak {
    fn run(&self) -> errors::Result<String> {
        let bytes = if self.as_hex {
            hex::decode(self.input.trim())?
        } else {
            self.input.as_bytes().to_vec()
        };
        Ok(format!("0x{}", hex::encode(keccak256(bytes))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_utf8_text() {
        let keccak = Keccak::new("hello".to_string(), false);
        assert_eq!(
            keccak.run().unwrap(),
            "0x1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8"
        );
    }

    #[test]
    fn empty_input_matches_known_digest() {
        let keccak = Keccak::new(String::new(), false);
        let empty = "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
        assert_eq!(keccak.run().unwrap(), empty);
        let keccak = Keccak::new("0x".to_string(), true);
        // Hashing an empty hex string hits the same all-empty input.
        assert_eq!(keccak.run().unwrap(), empty);
    }

    #[test]
    fn hex_mode_differs_from_text_mode() {
        // "0x68656c6c6f" is the hex for "hello", so hex mode must match text mode.
        let keccak1 = Keccak::new("0x68656c6c6f".to_string(), true);
        let keccak2 = Keccak::new("hello".to_string(), false);
        assert_eq!(keccak1.run().unwrap(), keccak2.run().unwrap());
    }

    #[test]
    fn errors_on_bad_hex() {
        let keccak = Keccak::new("0xzz".to_string(), true);
        assert!(keccak.run().is_err());
    }
}
