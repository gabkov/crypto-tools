//! `checksum` — print an address in its EIP-55 checksummed form.
//!
//! `Address::from_str` parses case-insensitively (with or without a `0x`
//! prefix), and `Address`'s `Display` is already EIP-55 checksummed, so we just
//! parse and re-print. Note this *normalizes* casing rather than *validating*
//! it: a wrongly-cased input is accepted, not rejected. (Use
//! `Address::parse_checksummed` if you instead want to reject bad casing.)

use std::str::FromStr;

use alloy_primitives::Address;

/// Entry point: parse `address` and return it in EIP-55 checksummed form.
pub fn run(address: &str) -> Result<String, String> {
    let parsed = Address::from_str(address).map_err(|e| format!("bad address input: {e}"))?;
    Ok(parsed.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checksums_lowercase_address() {
        assert_eq!(
            run("0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed").unwrap(),
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        );
    }

    #[test]
    fn accepts_address_without_0x_prefix() {
        assert_eq!(
            run("5aaeb6053f3e94c9b9a09f33669435e7ef1beaed").unwrap(),
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        );
    }

    #[test]
    fn errors_on_invalid_address() {
        assert!(run("0xnope").is_err());
    }
}
