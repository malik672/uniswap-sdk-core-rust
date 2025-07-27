# Uniswap SDK Core Rust

[![Rust CI](https://github.com/malik672/uniswap-sdk-core-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/malik672/uniswap-sdk-core-rust/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/uniswap-sdk-core)](https://docs.rs/uniswap-sdk-core/latest)
[![crates.io](https://img.shields.io/crates/v/uniswap-sdk-core.svg)](https://crates.io/crates/uniswap-sdk-core)

**A Custom Uniswap SDK Core in Rust provides essential functionality for interacting with the Uniswap decentralized
exchange.**

## Quickstart

Add this to your Cargo.toml

```toml
[dependencies]
uniswap-sdk-core = "5.2.0"
```

And this to your code:

```rust
use uniswap_sdk_core::prelude::*;
```

## Supported Rust Versions (MSRV)

<!--
When updating this, also update:
- clippy.toml
- Cargo.toml
- .github/workflows/rust.yml
-->

The current MSRV (minimum supported rust version) is 1.85.

## Note on `no_std`

By default, this library does not depend on the standard library (`std`). However, the `std` feature can be enabled.

## Examples

The code below shows an example of how to create a new `Token` instance for the DAI token on the Ethereum Mainnet using
the `token!` macro.

<details>
  <summary>Click to expand</summary>

```rust,ignore
// Import necessary preludes and token macro
use uniswap_sdk_core::{prelude::*, token};

// Define the chain ID, address, decimals, symbol, and name for the token
const CHAIN_ID: u64 = 1; // Ethereum Mainnet
const TOKEN_ADDRESS: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F"; // DAI Token Address
const DECIMALS: u8 = 18;
const SYMBOL: &str = "DAI";
const NAME: &str = "Dai Stablecoin";

// Use the `token!` macro to create a new `Token` instance
let dai_token = token!(CHAIN_ID, TOKEN_ADDRESS, DECIMALS, SYMBOL, NAME);

// Example usage of the `Token` methods
println!("Token Address: {}", dai_token.address());
println!("Is Native: {}", dai_token.is_native());

// Example of comparing two tokens
let another_dai_token = token!(CHAIN_ID, TOKEN_ADDRESS, DECIMALS, SYMBOL, NAME);
println!("Are the tokens equal? {}", dai_token.equals(&another_dai_token));

// Example of sorting tokens
let another_token = token!(CHAIN_ID, "0000000000000000000000000000000000000002", DECIMALS, "ETH", "Ethereum");
match dai_token.sorts_before( & another_token) {
Ok(true) => println ! ("DAI sorts before ETH"),
Ok(false) => println ! ("DAI does not sort before ETH"),
Err(e) => println ! ("Error comparing tokens: {:?}", e),
}
```

This example demonstrates how to create a `Token` instance for DAI on the Ethereum Mainnet using the `token!` macro.

It then prints the token's address and checks if it's a native token (which it isn't, so it prints false).

It also compares the DAI token with another DAI token instance to show that two instances of the same token are
considered equal.

Finally, it attempts to sort the DAI token before an Ethereum token, which should print that DAI sorts before ETH,
assuming the addresses are correctly set up for this comparison.

Remember to replace "0x6B175474E89094C44Da98b954EedeAC495271d0F" with the actual address of the DAI token you're working
with, and adjust the CHAIN_ID if you're working on a different network (e.g., a testnet).

</details>

## Contribution

Contributions are welcome! If you find a bug or have suggestions for improvements, feel free to open an issue or submit
a pull request on the [GitHub repository](https://github.com/malik672/uniswap-sdk-core-rust).

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgments

The Uniswap SDK Core in Rust is inspired by the original [Uniswap SDK](https://github.com/Uniswap/sdk-core) and aims to
provide similar functionality in the Rust programming language.

## Used by

- [Uniswap V3 SDK Rust](https://github.com/shuhuiluo/uniswap-v3-sdk-rs): Opinionated Rust implementation of the Uniswap
  V3 SDK with a focus on readability and performance
- [Uniswap V4 SDK Rust](https://github.com/shuhuiluo/uniswap-v4-sdk-rs): Opinionated Rust implementation of the Uniswap
  V4 SDK with a focus on readability and performance
- ...

*(If you want to add project to the list, dm or open a PR)*
