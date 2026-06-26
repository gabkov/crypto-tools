//! `decode` — turn raw hex calldata into a human-readable function call.

use alloy_dyn_abi::{DynSolValue, JsonAbiExt};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::hex;

use crate::{
    commands::Command,
    errors::{
        self,
        ToolError::{SelectorTooShort, UnknownSelector},
    },
};

use super::read_abi_file;

pub struct Decode<'a> {
    abi_path: &'a str,
    calldata: &'a str,
}

impl<'a> Decode<'a> {
    pub fn new(abi_path: &'a str, calldata: &'a str) -> Self {
        Decode { abi_path, calldata }
    }

    /// Decode `calldata_hex` against `abi_src` (the contents of an ABI JSON file)
    /// and return the call rendered as `name(type param = value, ...)`.
    fn decode_call(&self, abi_src: &str) -> errors::Result<String> {
        let abi: JsonAbi = serde_json::from_str(abi_src)?;

        // Decode the calldata hex (with or without a leading "0x").
        let calldata = hex::decode(self.calldata.trim())?;
        if calldata.len() < 4 {
            return Err(SelectorTooShort(
                "calldata is shorter than a 4-byte selector".to_string(),
            ));
        }
        let (selector, args_data) = calldata.split_at(4);

        // Find the function whose selector matches the first 4 bytes.
        let func = abi
            .functions()
            .find(|f| f.selector().as_slice() == selector)
            .ok_or_else(|| {
                UnknownSelector(format!(
                    "no function in ABI matches selector 0x{}",
                    hex::encode(selector)
                ))
            })?;

        // Decode the argument bytes against the function's input types.
        let values = func.abi_decode_input(args_data)?;

        Ok(render_call(func, &values))
    }
}

impl<'a> Command for Decode<'a> {
    fn run(&self) -> errors::Result<String> {
        let abi_src = read_abi_file(self.abi_path)?;
        self.decode_call(&abi_src)
    }
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
pub(crate) fn format_value(value: &DynSolValue) -> String {
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
    use crate::errors::ToolError;

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

        let decode = Decode::new("", calldata);

        let rendered = decode.decode_call(ERC20_ABI).unwrap();
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

        let decode = Decode::new("", calldata);
        assert!(decode.decode_call(ERC20_ABI).is_ok());
    }

    #[test]
    fn errors_on_unknown_selector() {
        let decode = Decode::new("", "0xdeadbeef");
        let err = decode.decode_call(ERC20_ABI).unwrap_err();
        assert!(matches!(err, ToolError::UnknownSelector(_)));
    }

    #[test]
    fn errors_on_short_calldata() {
        let decode = Decode::new("", "0xa905");
        let err = decode.decode_call(ERC20_ABI).unwrap_err();
        assert!(matches!(err, ToolError::SelectorTooShort(_)));
    }
}
