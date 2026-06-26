//! `convert` — convert an amount between Ethereum units.
//!
//! Units are whatever alloy accepts (wei, gwei, ether, and aliases like eth,
//! shannon, szabo). Conversion goes through wei internally, so it's exact.

use alloy_primitives::utils::{format_units, parse_units};

use crate::{commands::Command, errors};

pub struct Convert {
    value: String,
    from: String,
    to: String,
}

impl Convert {
    pub fn new(value: String, from: String, to: String) -> Self {
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

impl Command for Convert {
    fn run(&self) -> errors::Result<String> {
        let amount = parse_units(&self.value, &self.from)?;
        let formatted = format_units(amount, &self.to)?;
        Ok(Convert::trim_trailing_zeros(formatted))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ether_to_wei() {
        let convert = Convert::new("1".to_string(), "ether".to_string(), "wei".to_string());
        assert_eq!(convert.run().unwrap(), "1000000000000000000");
    }

    #[test]
    fn wei_to_gwei() {
        let convert = Convert::new(
            "1000000000".to_string(),
            "wei".to_string(),
            "gwei".to_string(),
        );
        assert_eq!(convert.run().unwrap(), "1");
    }

    #[test]
    fn fractional_ether() {
        let convert = Convert::new("2.5".to_string(), "ether".to_string(), "gwei".to_string());
        assert_eq!(convert.run().unwrap(), "2500000000");
    }

    #[test]
    fn preserves_significant_fraction() {
        let convert = Convert::new(
            "1234567890".to_string(),
            "wei".to_string(),
            "gwei".to_string(),
        );
        // Trimming must not eat meaningful digits.
        assert_eq!(convert.run().unwrap(), "1.23456789");
    }

    #[test]
    fn errors_on_unknown_unit() {
        let convert = Convert::new("1".to_string(), "ether".to_string(), "dollars".to_string());
        assert!(convert.run().is_err());
    }
}
