use alloy_primitives::{AddressError, hex, utils::UnitsError};

#[derive(Debug)]
pub enum ToolError {
    BadHex(String),
    InvalidFunctionSignature(String),
    InvalidAbiJson(String),
    InvalidFilePath(String),
    SelectorTooShort(String),
    UnknownSelector(String),
    FunctionNotFound(String),
    InvalidArguments(String),
    InvalidConvertParams(String),
    InvalidChecksum(String),
}

impl From<hex::FromHexError> for ToolError {
    fn from(value: hex::FromHexError) -> Self {
        ToolError::BadHex(format!("bad hex input: {value}"))
    }
}

impl From<alloy_json_abi::parser::Error> for ToolError {
    fn from(value: alloy_json_abi::parser::Error) -> Self {
        ToolError::InvalidFunctionSignature(format!("invalid function signature: {value}"))
    }
}

impl From<std::io::Error> for ToolError {
    fn from(value: std::io::Error) -> Self {
        ToolError::InvalidFilePath(format!("could not read file: {value}"))
    }
}

impl From<serde_json::Error> for ToolError {
    fn from(value: serde_json::Error) -> Self {
        ToolError::InvalidAbiJson(format!("parsing ABI JSON: {value}"))
    }
}

impl From<alloy_dyn_abi::Error> for ToolError {
    fn from(value: alloy_dyn_abi::Error) -> Self {
        ToolError::InvalidArguments(format!("invalid arguments: {value}"))
    }
}

impl From<UnitsError> for ToolError {
    fn from(value: UnitsError) -> Self {
        ToolError::InvalidConvertParams(format!("convert error: {value}"))
    }
}

impl From<AddressError> for ToolError {
    fn from(value: AddressError) -> Self {
        ToolError::InvalidChecksum(format!("invalid EIP-55 checksum: {value}"))
    }
}

impl std::error::Error for ToolError {}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ToolError::BadHex(m)
            | ToolError::InvalidFunctionSignature(m)
            | ToolError::InvalidAbiJson(m)
            | ToolError::InvalidFilePath(m)
            | ToolError::SelectorTooShort(m)
            | ToolError::UnknownSelector(m)
            | ToolError::FunctionNotFound(m)
            | ToolError::InvalidArguments(m)
            | ToolError::InvalidConvertParams(m)
            | ToolError::InvalidChecksum(m) => m,
        };
        write!(f, "{msg}")
    }
}

pub type Result<T> = std::result::Result<T, ToolError>;
