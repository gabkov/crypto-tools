//! crypto-tools — an EVM Swiss-army knife.
//!
//! Commands:
//!     decode    <abi.json> <calldata-hex>      decode calldata into a call
//!     encode    <abi.json> <function> [args]   encode a call into calldata
//!     selector  "<signature>"                  4-byte selector of a signature
//!     keccak    [--hex] <input>                keccak-256 hash of some data
//!     keygen                                   random private key + address
//!     convert   <value> <from> <to>            convert between ETH units
//!     checksum  <address>                      EIP-55 checksum an address
//!
//! Run `cargo run -- <command> --help` for details on any command.

mod cli;
mod commands;
mod errors;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Decode { abi, calldata } => commands::decode::run(&abi, &calldata),
        Commands::Encode {
            abi,
            function,
            args,
        } => commands::encode::run(&abi, &function, &args),
        Commands::Selector { signature } => commands::selector::run(&signature),
        Commands::Keygen => commands::keygen::run(),
        Commands::Keccak { input, hex } => commands::keccak::run(&input, hex),
        Commands::Convert { value, from, to } => commands::convert::run(&value, &from, &to),
        Commands::Checksum { address } => commands::checksum::run(&address),
    };

    match result {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
