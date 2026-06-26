//! `encode` — turn a function name + string args into hex calldata.

use alloy_dyn_abi::{DynSolType, JsonAbiExt, Specifier};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::hex;

use crate::{
    commands::Command,
    errors::{
        self,
        ToolError::{FunctionNotFound, InvalidArguments},
    },
};

use super::read_abi_file;

pub struct Encode<'a> {
    abi_path: &'a str,
    fn_name: &'a str,
    args: &'a [String],
}

impl<'a> Encode<'a> {
    pub fn new(abi_path: &'a str, fn_name: &'a str, args: &'a [String]) -> Self {
        Encode {
            abi_path,
            fn_name,
            args,
        }
    }

    /// Encode a call to `fn_name` with `args` (raw strings) against `abi_src`,
    /// returning the full calldata as a `0x`-prefixed hex string.
    fn encode_call(&self, abi_src: &str) -> errors::Result<String> {
        let abi: JsonAbi = serde_json::from_str(abi_src)?;

        // Among same-named overloads, pick the one whose arity matches the args.
        let candidates: Vec<&Function> =
            abi.functions().filter(|f| f.name == self.fn_name).collect();
        let expected_arity = candidates.first().map(|f| f.inputs.len());
        let func = candidates
            .into_iter()
            .find(|f| f.inputs.len() == self.args.len())
            .ok_or_else(|| match expected_arity {
                None => FunctionNotFound(format!("no function named '{0}' in ABI", self.fn_name)),
                Some(n) => InvalidArguments(format!(
                    "function '{0}' expects {n} argument(s), got {1}",
                    self.fn_name,
                    self.args.len()
                )),
            })?;

        // Coerce each string arg into a typed value matching the parameter type.
        let mut values = Vec::with_capacity(self.args.len());
        for (input, arg) in func.inputs.iter().zip(self.args) {
            let ty: DynSolType = input.resolve()?;
            let value = ty.coerce_str(arg)?;
            values.push(value);
        }

        // `abi_encode_input` prepends the 4-byte selector, giving full calldata.
        let encoded = func.abi_encode_input(&values)?;

        Ok(format!("0x{}", hex::encode(encoded)))
    }
}

impl<'a> Command for Encode<'a> {
    fn run(&self) -> errors::Result<String> {
        let abi_src = read_abi_file(self.abi_path)?;
        self.encode_call(&abi_src)
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
    fn encodes_erc20_transfer() {
        let args = [
            "0x000000000000000000000000000000000000abc0".to_string(),
            "10".to_string(),
        ];
        let encode = Encode::new("", "transfer", &args);
        let calldata = encode.encode_call(ERC20_ABI).unwrap();
        assert_eq!(
            calldata,
            "0xa9059cbb\
             000000000000000000000000000000000000000000000000000000000000abc0\
             000000000000000000000000000000000000000000000000000000000000000a"
        );
    }

    #[test]
    fn errors_on_unknown_function() {
        let encode = Encode::new("", "mint", &[]);
        let err = encode.encode_call(ERC20_ABI).unwrap_err();
        assert!(matches!(err, ToolError::FunctionNotFound(_)));
    }

    #[test]
    fn errors_on_wrong_arity() {
        let args = ["0x000000000000000000000000000000000000abc0".to_string()];
        let encode = Encode::new("", "transfer", &args);
        let err = encode.encode_call(ERC20_ABI).unwrap_err();
        assert!(matches!(err, ToolError::InvalidArguments(_)));
    }
}
