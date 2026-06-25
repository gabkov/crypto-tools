//! Subcommand implementations. Each command exposes a `run(...)` entry point
//! and keeps its testable core working on an in-memory ABI string.

pub mod convert;
pub mod decode;
pub mod encode;
pub mod keccak;
pub mod keygen;
pub mod selector;

/// Read an ABI JSON file into a string, mapping IO errors to a message.
pub(crate) fn read_abi_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| format!("reading {path}: {e}"))
}
