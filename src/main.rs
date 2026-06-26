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

use crate::commands::{
    Command, checksum::Checksum, convert::Convert, decode::Decode, encode::Encode, keccak::Keccak,
    keygen::Keygen, selector::Selector,
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result: Box<dyn Command> = match cli.command {
        Commands::Decode { abi, calldata } => Box::new(Decode::new(abi, calldata)),
        Commands::Encode {
            abi,
            function,
            args,
        } => Box::new(Encode::new(abi, function, args)),
        Commands::Selector { signature } => Box::new(Selector::new(signature)),
        Commands::Keygen => Box::new(Keygen::new()),
        Commands::Keccak { input, hex } => Box::new(Keccak::new(input, hex)),
        Commands::Convert { value, from, to } => Box::new(Convert::new(value, from, to)),
        Commands::Checksum { address } => Box::new(Checksum::new(address)),
    };

    match result.run() {
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
