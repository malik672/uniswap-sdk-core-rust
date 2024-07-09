//! # uniswap-sdk-core
//!
//! The Uniswap SDK Core in Rust provides essential functionality for interacting with the Uniswap
//! decentralized exchange.
#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    clippy::missing_const_for_fn,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
extern crate alloc;

/// Contains functionality related to All Contracts deployed and supported by the Uniswap SDK.
pub mod addresses;
/// Contains functionality related to All Contracts deployed and supported by the Uniswap SDK.
pub mod chains;
/// Contains some constants and enums used in the Uniswap SDK Core
pub mod constants;
/// Contains entities related to the Uniswap SDK Core, such as currencies, tokens, and fractions.
pub mod entities;
/// Contains error types  for the Uniswap SDK Core.
///
/// This module defines custom error types that are used throughout the SDK to
/// handle various error conditions.
pub mod error;
/// Contains commonly used items from the Uniswap SDK Core.
///
/// This module re-exports items that are commonly used together,
/// making it easier to import them in other parts of your application.
pub mod prelude;
/// Contains utility functions and helpers used across the Uniswap SDK Core.
pub mod utils;

/// Contains examples of how Uniswap sdk core can be used
pub mod examples;
