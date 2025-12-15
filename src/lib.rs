#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    clippy::missing_const_for_fn,
    clippy::missing_inline_in_public_items,
    clippy::needless_pass_by_value,
    clippy::redundant_clone,
    clippy::manual_assert,
    clippy::must_use_candidate,
    clippy::unseparated_literal_suffix,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;
use num_traits as _;

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
/// Contains utility functions and helpers used across the Uniswap SDK Core.
pub mod utils;

/// Contains commonly used items from the Uniswap SDK Core.
///
/// This module re-exports items that are commonly used together,
/// making it easier to import them in other parts of your application.
pub mod prelude {
    pub use crate::{addresses::*, chains::*, constants::*, entities::*, error::Error, utils::*};

    pub use alloc::{
        string::{String, ToString},
        vec::Vec,
    };
    pub use alloy_primitives::{Address, B256, Bytes, U256, map::HashMap};
    pub use fastnum;
    pub use num_integer::Integer;

    pub type BigInt = fastnum::I512;
    pub type BigUint = fastnum::U512;
    pub type BigDecimal = fastnum::D512;
    pub type RoundingMode = fastnum::decimal::RoundingMode;
}

/// Contains examples of how Uniswap sdk core can be used
#[cfg(all(feature = "std", test))]
mod examples;
