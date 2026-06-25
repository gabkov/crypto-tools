# crypto-tools

A small EVM Swiss-army knife CLI, built on [alloy](https://github.com/alloy-rs/alloy) as a Rust learning project. Encode/decode calldata, compute selectors and hashes, generate keys, and convert between units.

## Build

```sh
cargo build
cargo test
```

## Commands

Run via `cargo run -- <command>` (or the built binary at `target/debug/crypto-tools`).

| Command | Description |
| --- | --- |
| `decode <abi.json> <calldata-hex>` | Decode hex calldata into a human-readable call |
| `encode <abi.json> <function> [args...]` | Encode a function call into hex calldata |
| `selector "<signature>"` | Compute the 4-byte selector of a function signature |
| `keccak [--hex] <input>` | Keccak-256 hash of UTF-8 text (or hex bytes with `--hex`) |
| `keygen` | Generate a random private key and its address |
| `convert <value> <from> <to>` | Convert an amount between ETH units (wei, gwei, ether, …) |
| `checksum <address>` | Print an address in EIP-55 checksummed form (validates mixed-case input) |

### Examples

```sh
# Decode calldata
cargo run -- decode erc20.json 0xa9059cbb000...000a
# -> transfer(address _to = 0x...ABC0, uint256 _value = 10)

# Encode a call
cargo run -- encode erc20.json transfer 0x...abc0 10
# -> 0xa9059cbb000...000a

# Function selector
cargo run -- selector "transfer(address,uint256)"
# -> 0xa9059cbb

# Hash an event signature (its topic0)
cargo run -- keccak "Transfer(address,address,uint256)"

# Generate a throwaway keypair
cargo run -- keygen

# Convert between units
cargo run -- convert 1 ether wei
# -> 1000000000000000000

# Checksum an address (lowercase -> canonical casing)
cargo run -- checksum 0x5aaeb6053f3e94c9b9a09f33669435e7ef1beaed
# -> 0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed
# A mixed-case address with a typo'd letter is rejected as an invalid checksum.
```

## Project layout

```
src/
  main.rs            entry: parse CLI, dispatch, exit codes
  cli.rs             clap command definitions
  commands/          one module per command (+ unit tests)
```

To add a command: add a variant in `cli.rs`, a `commands/<name>.rs` with a
`run(...)`, declare it in `commands/mod.rs`, and add a match arm in `main.rs`.

> ⚠️ `keygen` prints the private key to stdout — fine for test accounts, not for
> securing real funds.
