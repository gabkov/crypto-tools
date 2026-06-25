//! `convert` — convert an amount between Ethereum units.
//!
//! Units are whatever alloy accepts (wei, gwei, ether, and aliases like eth,
//! shannon, szabo). Conversion goes through wei internally, so it's exact.

use alloy_primitives::utils::{format_units, parse_units};

use crate::errors;

/// Entry point: parse `value` in the `from` unit and re-express it in `to`.
pub fn run(value: &str, from: &str, to: &str) -> errors::Result<String> {
    let amount = parse_units(value, from)?;
    let formatted = format_units(amount, to)?;
    Ok(trim_trailing_zeros(formatted))
}

/// `format_units` always prints a decimal point and full precision (e.g.
/// "1.000000000"); strip insignificant trailing zeros for readable output.
fn trim_trailing_zeros(s: String) -> String {
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ether_to_wei() {
        assert_eq!(run("1", "ether", "wei").unwrap(), "1000000000000000000");
    }

    #[test]
    fn wei_to_gwei() {
        assert_eq!(run("1000000000", "wei", "gwei").unwrap(), "1");
    }

    #[test]
    fn fractional_ether() {
        assert_eq!(run("2.5", "ether", "gwei").unwrap(), "2500000000");
    }

    #[test]
    fn preserves_significant_fraction() {
        // Trimming must not eat meaningful digits.
        assert_eq!(run("1234567890", "wei", "gwei").unwrap(), "1.23456789");
    }

    #[test]
    fn errors_on_unknown_unit() {
        assert!(run("1", "ether", "dollars").is_err());
    }
}
