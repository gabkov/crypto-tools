use alloy::{
    primitives::{AddressError, hex, utils::UnitsError},
    transports::{RpcError, TransportErrorKind},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolError {
    // `#[from]` generates the `From` impl AND records the field as the
    // error source (restoring the chain). `{0}` in `#[error]` is that field,
    // printed via its own Display.
    #[error("bad hex input: {0}")]
    BadHex(#[from] hex::FromHexError),

    #[error("invalid function signature: {0}")]
    InvalidFunctionSignature(#[from] alloy::json_abi::parser::Error),

    #[error("parsing ABI JSON: {0}")]
    InvalidAbiJson(#[from] serde_json::Error),

    #[error("invalid file path: {0}")]
    InvalidFilePath(#[from] std::io::Error),

    #[error("invalid arguments: {0}")]
    AbiCoding(#[from] alloy::dyn_abi::Error),

    #[error("invalid convert params: {0}")]
    InvalidConvertParams(#[from] UnitsError),

    #[error("invalid EIP-55 checksum: {0}")]
    InvalidChecksum(#[from] AddressError),

    #[error("invalid RPC url: {0}")]
    InvalidRpcUrl(#[from] url::ParseError),

    #[error("invalid balance request: {0}")]
    InvalidBalanceRequest(#[from] RpcError<TransportErrorKind>),

    // Manually-constructed messages stay as `String`; `{0}` just prints it.
    #[error("{0}")]
    SelectorTooShort(String),

    #[error("{0}")]
    UnknownSelector(String),

    #[error("{0}")]
    FunctionNotFound(String),

    #[error("{0}")]
    InvalidArguments(String),
}

pub type Result<T> = std::result::Result<T, ToolError>;
