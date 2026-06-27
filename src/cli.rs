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
    /// Compute the 4-byte selector of a function signature.
    Selector {
        /// Function signature, e.g. "transfer(address,uint256)".
        signature: String,
    },
    /// Generate a random private key and its Ethereum address.
    Keygen,
    /// Compute the keccak-256 hash of some data.
    Keccak {
        /// Data to hash (UTF-8 text by default).
        input: String,
        /// Interpret the input as hex bytes instead of UTF-8 text.
        #[arg(long)]
        hex: bool,
    },
    /// Convert an amount between Ethereum units (e.g. wei, gwei, ether).
    Convert {
        /// Amount to convert, in the source unit (e.g. 1.5).
        value: String,
        /// Source unit (e.g. ether).
        from: String,
        /// Target unit (e.g. wei).
        to: String,
    },
    /// Print an address in its EIP-55 checksummed form.
    Checksum {
        /// Address to checksum (with or without a `0x` prefix).
        address: String,
    },
    /// Polls the current balance address from ethereum or any supplied RPC/Chain (must be EVM compatible)
    Balance {
        /// Address to get the balance for.
        address: String,
        /// RPC endpoint to query.
        #[arg(long, default_value = "https://rpc.mevblocker.io")]
        rpc_url: String,
    },
}
