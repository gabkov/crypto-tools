//! `convert` — convert an amount between Ethereum units.
//!
//! Units are whatever alloy accepts (wei, gwei, ether, and aliases like eth,
//! shannon, szabo). Conversion goes through wei internally, so it's exact.

use alloy_primitives::utils::{format_units, parse_units};

use crate::{commands::Command, errors};

pub struct Convert<'a> {
    value: &'a str,
    from: &'a str,
    to: &'a str,
}

impl<'a> Convert<'a> {
    pub fn new(value: &'a str, from: &'a str, to: &'a str) -> Self {
        Convert { value, from, to }
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
}

impl<'a> Command for Convert<'a> {
    fn run(&self) -> errors::Result<String> {
        let amount = parse_units(self.value, self.from)?;
        let formatted = format_units(amount, self.to)?;
        Ok(Convert::trim_trailing_zeros(formatted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ether_to_wei() {
        let convert = Convert::new("1", "ether", "wei");
        assert_eq!(convert.run().unwrap(), "1000000000000000000");
    }

    #[test]
    fn wei_to_gwei() {
        let convert = Convert::new("1000000000", "wei", "gwei");
        assert_eq!(convert.run().unwrap(), "1");
    }

    #[test]
    fn fractional_ether() {
        let convert = Convert::new("2.5", "ether", "gwei");
        assert_eq!(convert.run().unwrap(), "2500000000");
    }

    #[test]
    fn preserves_significant_fraction() {
        let convert = Convert::new("1234567890", "wei", "gwei");
        // Trimming must not eat meaningful digits.
        assert_eq!(convert.run().unwrap(), "1.23456789");
    }

    #[test]
    fn errors_on_unknown_unit() {
        let convert = Convert::new("1", "ether", "dollars");
        assert!(convert.run().is_err());
    }
}
