//! The command-line surface, defined declaratively with `clap`.

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "crypto-tools", about = "An EVM calldata Swiss-army knife")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Decode hex calldata against an ABI into a human-readable call.
    Decode {
        /// Path to the contract ABI JSON file.
        abi: String,
        /// Hex calldata, with or without a leading `0x`.
        calldata: String,
    },
    /// Encode a function call into hex calldata.
    Encode {
        /// Path to the contract ABI JSON file.
        abi: String,
        /// Name of the function to call.
        function: String,
        /// Function arguments, in declaration order (e.g. 0xabc... 10).
        args: Vec<String>,
    },
}
