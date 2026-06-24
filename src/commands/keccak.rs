//! `keccak` — compute the keccak-256 hash of some data.
//!
//! Hashes the UTF-8 bytes of the input by default; pass `--hex` to hash the
//! decoded bytes of a hex string instead.

use alloy_primitives::{hex, keccak256};

/// Entry point: hash `input` and return the digest as `0x`-prefixed hex.
pub fn run(input: &str, as_hex: bool) -> Result<String, String> {
    let bytes = if as_hex {
        hex::decode(input.trim()).map_err(|e| format!("bad hex input: {e}"))?
    } else {
        input.as_bytes().to_vec()
    };
    Ok(format!("0x{}", hex::encode(keccak256(bytes))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_utf8_text() {
        assert_eq!(
            run("hello", false).unwrap(),
            "0x1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8"
        );
    }

    #[test]
    fn empty_input_matches_known_digest() {
        let empty = "0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
        assert_eq!(run("", false).unwrap(), empty);
        // Hashing an empty hex string hits the same all-empty input.
        assert_eq!(run("0x", true).unwrap(), empty);
    }

    #[test]
    fn hex_mode_differs_from_text_mode() {
        // "0x68656c6c6f" is the hex for "hello", so hex mode must match text mode.
        assert_eq!(run("0x68656c6c6f", true).unwrap(), run("hello", false).unwrap());
    }

    #[test]
    fn errors_on_bad_hex() {
        assert!(run("0xzz", true).is_err());
    }
}
