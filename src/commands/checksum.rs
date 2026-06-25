//! `checksum` — print an address in its EIP-55 checksummed form.
//!
//! `Address`'s `Display` is already EIP-55 checksummed, so producing the output
//! is just parse-and-reprint. The casing of the *input* decides how we parse:
//!
//! - Single-case input (all-lower or all-upper) asserts no checksum, so we
//!   normalize it with `Address::from_str`.
//! - Mixed-case input is asserting an EIP-55 checksum, so we verify it with
//!   `Address::parse_checksummed` and reject it if the casing is wrong — which
//!   is how a single-character typo gets caught.
//!
//! Both `0x`-prefixed and bare input are accepted.

use std::str::FromStr;

use alloy_primitives::Address;

/// Entry point: parse `address` and return it in EIP-55 checksummed form.
///
/// A single-case address carries no checksum, so we just normalize it. A
/// mixed-case address is asserting an EIP-55 checksum, so we verify that casing
/// and reject it if it's wrong (catching typos).
pub fn run(address: &str) -> Result<String, String> {
    // `from_str` tolerates a missing `0x`, but `parse_checksummed` requires it,
    // so normalize to one prefixed form and feed that to both paths.
    let body = address.strip_prefix("0x").unwrap_or(address);
    let prefixed = format!("0x{body}");

    // Only the hex letters (a–f) carry case, so judge casing on those alone.
    let is_lower = body
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(char::is_lowercase);
    let is_upper = body
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(char::is_uppercase);

    let parsed = if is_lower || is_upper {
        Address::from_str(&prefixed).map_err(|e| format!("bad address input: {e}"))?
    } else {
        Address::parse_checksummed(&prefixed, None)
            .map_err(|e| format!("invalid EIP-55 checksum: {e}"))?
    };

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

    #[test]
    fn normalizes_uppercase_address() {
        assert_eq!(
            run("0x5AAEB6053F3E94C9B9A09F33669435E7EF1BEAED").unwrap(),
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        );
    }

    #[test]
    fn accepts_correctly_checksummed_address() {
        let checksummed = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";
        assert_eq!(run(checksummed).unwrap(), checksummed);
    }

    #[test]
    fn rejects_wrongly_checksummed_address() {
        // Same address as above with one letter's case flipped (last char).
        let typo = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAeD";
        let err = run(typo).unwrap_err();
        assert!(err.contains("invalid EIP-55 checksum"));
    }

    #[test]
    fn validates_mixed_case_without_0x_prefix() {
        // Mixed-case input still gets checksum-validated even without the prefix.
        assert_eq!(
            run("5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed").unwrap(),
            "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed"
        );
    }
}
