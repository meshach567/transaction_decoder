# Bitcoin Transaction Decoder

This Rust project decodes and parses raw Bitcoin transaction hex strings, extracting and displaying their components in a human-readable JSON format.

## Features

- Parses Bitcoin transaction hex strings.
- Decodes transaction version, inputs, outputs, and scripts.
- Converts satoshi amounts to BTC.
- Outputs the parsed transaction as pretty-printed JSON.
- Includes unit tests for compact size integer parsing.

## Example

Given a raw transaction hex, the program will output its structured contents:

```json
{
  "version": 1,
  "inputs": [
    {
      "txid": "4ac541802679866935a19d4f40728bb89204d0cac90d85f3a51a19278fe33aeb",
      "output_index": 1,
      "script_sig": "",
      "sequence": 4294967295
    }
  ],
  "outputs": [
    {
      "amount": 0.02000000,
      "script_pubkey": "51203b41daba4c9ace578369740f15e5ec880c28279ee7f51b07dca69c7061e07068f"
    },
    {
      "amount": 0.02400000,
      "script_pubkey": "00147752c165ea7be772b2c0acb7f4d6047ae6f4768e"
    }
  ]
}
```

## Usage

1. **Clone the repository:**
   ```sh
   git clone https://github.com/meshach567/transaction_decoder.git
   cd transaction_decoder
   ```

2. **Build and run:**
   ```sh
   cargo run
   ```

   The main function contains a sample transaction hex string. You can modify this string in `src/main.rs` to decode other transactions.

## Project Structure

- `src/main.rs`: Main logic for parsing and decoding Bitcoin transactions.
- `Cargo.toml`: Project manifest.

## Testing

Unit tests are provided for the compact size integer parsing:

```sh
cargo test
```

## Dependencies

- [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) for JSON serialization.
- [hex](https://crates.io/crates/hex) for hex decoding.

Dependencies are managed via Cargo and specified in `Cargo.toml`.

## License

MIT License