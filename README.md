# Currency Converter

**A CLI based currency converter using rust. (For learning purpose)**

This, CLI app is used to convert currency. Inorder, to convert it sends a request to (https://latest.currency-api.pages.dev/v1/currencies/) from which it retrieves the .json of a currency and it's respective exchange rate for other currency.

## Usage
Compile the program first:
`cargo build --release`

Then go inside target/release/ folder
`cd target/release`

Run as:
`./currency-converter <from_currency> <to_currency> <amount>`

**Example**:
`./currency-converter usd aud 50`

## Crates Used
- [tokio](https://crates.io/crates/tokio)
- [reqwest](https://crates.io/crates/reqwest)
- [serde_json](https://crates.io/crates/serde_json)

## Credits
API made by: https://github.com/fawazahmed0
