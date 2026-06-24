//! A tiny ABI decoder CLI.
//!
//! Usage:
//!     cargo run -- <abi.json> <calldata-hex>
//!     ./target/debug/crypto-tools <abi.json> <calldata-hex>   (after `cargo build`)
//!
//! Reads a contract ABI, matches the 4-byte selector at the start of the
//! calldata against the ABI's functions, decodes the arguments, and prints
//! the call in a human-readable form, e.g.
//!     transfer(address _to = 0xabc..., uint256 _value = 10)

use std::process::ExitCode;

use alloy_dyn_abi::{DynSolValue, JsonAbiExt};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::hex;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <abi.json> <calldata-hex>", args[0]);
        return ExitCode::FAILURE;
    }

    let abi_src = match std::fs::read_to_string(&args[1]) {
        Ok(src) => src,
        Err(e) => {
            eprintln!("error: reading {}: {e}", args[1]);
            return ExitCode::FAILURE;
        }
    };

    match decode_call(&abi_src, &args[2]) {
        Ok(rendered) => {
            println!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

/// Decode `calldata_hex` against `abi_src` (the contents of an ABI JSON file)
/// and return the call rendered as `name(type param = value, ...)`.
fn decode_call(abi_src: &str, calldata_hex: &str) -> Result<String, String> {
    let abi: JsonAbi =
        serde_json::from_str(abi_src).map_err(|e| format!("parsing ABI JSON: {e}"))?;

    // Decode the calldata hex (with or without a leading "0x").
    let calldata =
        hex::decode(calldata_hex.trim()).map_err(|e| format!("bad hex calldata: {e}"))?;
    if calldata.len() < 4 {
        return Err("calldata is shorter than a 4-byte selector".into());
    }
    let (selector, args_data) = calldata.split_at(4);

    // Find the function whose selector matches the first 4 bytes.
    let func = abi
        .functions()
        .find(|f| f.selector().as_slice() == selector)
        .ok_or_else(|| {
            format!(
                "no function in ABI matches selector 0x{}",
                hex::encode(selector)
            )
        })?;

    // Decode the argument bytes against the function's input types.
    let values = func
        .abi_decode_input(args_data)
        .map_err(|e| format!("decoding arguments: {e}"))?;

    Ok(render_call(func, &values))
}

/// Render a decoded call as `name(type param = value, ...)`.
fn render_call(func: &Function, values: &[DynSolValue]) -> String {
    let args: Vec<String> = func
        .inputs
        .iter()
        .zip(values)
        .map(|(input, value)| {
            let name = if input.name.is_empty() {
                String::new()
            } else {
                format!(" {}", input.name)
            };
            format!("{}{} = {}", input.ty, name, format_value(value))
        })
        .collect();
    format!("{}({})", func.name, args.join(", "))
}

/// Format a single decoded value for display.
fn format_value(value: &DynSolValue) -> String {
    match value {
        DynSolValue::Address(a) => a.to_string(),
        DynSolValue::Bool(b) => b.to_string(),
        DynSolValue::Uint(n, _) => n.to_string(),
        DynSolValue::Int(n, _) => n.to_string(),
        DynSolValue::String(s) => format!("{s:?}"),
        DynSolValue::Bytes(b) => format!("0x{}", hex::encode(b)),
        DynSolValue::FixedBytes(b, size) => format!("0x{}", hex::encode(&b[..*size])),
        DynSolValue::Function(f) => f.to_string(),
        DynSolValue::Array(items) | DynSolValue::FixedArray(items) => {
            let inner: Vec<String> = items.iter().map(format_value).collect();
            format!("[{}]", inner.join(", "))
        }
        DynSolValue::Tuple(items) => {
            let inner: Vec<String> = items.iter().map(format_value).collect();
            format!("({})", inner.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERC20_ABI: &str = r#"[
        {"type":"function","name":"transfer","inputs":[
            {"name":"_to","type":"address"},
            {"name":"_value","type":"uint256"}
        ],"outputs":[{"name":"","type":"bool"}],"stateMutability":"nonpayable"}
    ]"#;

    #[test]
    fn decodes_erc20_transfer() {
        // transfer(0x...ABC0, 10)
        let calldata = "0xa9059cbb\
            000000000000000000000000000000000000000000000000000000000000abc0\
            000000000000000000000000000000000000000000000000000000000000000a";

        let rendered = decode_call(ERC20_ABI, calldata).unwrap();
        assert_eq!(
            rendered,
            "transfer(address _to = 0x000000000000000000000000000000000000ABC0, uint256 _value = 10)"
        );
    }

    #[test]
    fn accepts_calldata_without_0x_prefix() {
        let calldata = "a9059cbb\
            000000000000000000000000000000000000000000000000000000000000abc0\
            000000000000000000000000000000000000000000000000000000000000000a";
        assert!(decode_call(ERC20_ABI, calldata).is_ok());
    }

    #[test]
    fn errors_on_unknown_selector() {
        let err = decode_call(ERC20_ABI, "0xdeadbeef").unwrap_err();
        assert!(err.contains("no function in ABI matches selector 0xdeadbeef"));
    }

    #[test]
    fn errors_on_short_calldata() {
        let err = decode_call(ERC20_ABI, "0xa905").unwrap_err();
        assert!(err.contains("shorter than a 4-byte selector"));
    }
}
