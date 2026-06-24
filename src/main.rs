//! crypto-tools — an EVM calldata Swiss-army knife.
//!
//! Usage:
//!     cargo run -- <command> [args...]
//!     cargo run -- decode <abi.json> <calldata-hex>
//!     cargo run -- encode <abi.json> <function> [args...]
//!
//! Run `cargo run -- --help` for the full command list.

mod cli;
mod commands;

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
