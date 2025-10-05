use crate::prelude::*;

/// Represents an ERC20 token with a unique address and some metadata.
pub type Token = CurrencyLike<false, TokenMeta>;

/// Represents the metadata for an ERC20 token, including its address and optional fees.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct TokenMeta {
    /// The address of the token.
    pub address: Address,
    /// The buy fee in basis points (bps) for the token.
    pub buy_fee_bps: u64,
    /// The sell fee in basis points (bps) for the token.
    pub sell_fee_bps: u64,
}

macro_rules! impl_base_currency {
    ($($token:ty),*) => {
        $(
            impl BaseCurrency for $token {
                #[inline]
                fn equals(&self, other: &impl BaseCurrency) -> bool {
                    other.is_token() && self.chain_id == other.chain_id() && self.address == other.address()
                }

                #[inline]
                fn wrapped(&self) -> &Token {
                    self
                }
            }
        )*
    };
}

impl_base_currency!(Token, &Token);

impl Token {
    /// Creates a new [`Token`] with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `chain_id` - The chain ID of the token.
    /// * `address` - The address of the token.
    /// * `decimals` - The number of decimals the token uses.
    /// * `symbol` - The symbol of the token, if any.
    /// * `name` - The name of the token, if any.
    /// * `buy_fee_bps` - The buy fee in basis points (bps), if any.
    /// * `sell_fee_bps` - The sell fee in basis points (bps), if any.
    ///
    /// # Returns
    ///
    /// A new [`Token`] instance.
    ///
    /// # Panics
    ///
    /// Panics if `chain_id` is 0.
    #[inline]
    #[must_use]
    pub const fn new(
        chain_id: u64,
        address: Address,
        decimals: u8,
        symbol: Option<String>,
        name: Option<String>,
        buy_fee_bps: u64,
        sell_fee_bps: u64,
    ) -> Self {
        assert!(chain_id != 0, "chain id can't be zero");
        Self {
            chain_id,
            decimals,
            symbol,
            name,
            meta: TokenMeta {
                address,
                buy_fee_bps,
                sell_fee_bps,
            },
        }
    }

    /// Returns true if the address of this token sorts before the address of the other token.
    /// Panics if the tokens have the same address or if the tokens are on different chains.
    ///
    /// # Arguments
    ///
    /// * `other`: another token to compare
    #[inline]
    pub fn sorts_before(&self, other: &Token) -> Result<bool, Error> {
        if self.chain_id != other.chain_id {
            return Err(Error::ChainIdMismatch(self.chain_id, other.chain_id));
        }

        if self.address() == other.address() {
            return Err(Error::EqualAddresses);
        }
        Ok(self.address() < other.address())
    }
}

/// Shorthand macro to create a [`Token`] with the given chain id, address, decimals, optional
/// symbol and name.
///
/// # Arguments
///
/// * `chain_id`: The chain id
/// * `address`: The address of the token as a string, [`Address`] or a string literal without "0x"
/// * `decimals`: The decimals of the token
/// * `symbol`: The symbol of the token, optional
/// * `name`: The name of the token, optional
///
/// returns: [`Token`]
///
/// # Example
///
/// ```
/// use uniswap_sdk_core::{prelude::*, token};
///
/// const DAI_MAINNET: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
/// let dai: Token = token!(1, DAI_MAINNET, 18, "DAI", "Dai Stablecoin");
/// let dai: Token = token!(
///     1,
///     "6B175474E89094C44Da98b954EedeAC495271d0F",
///     18,
///     "DAI",
///     "Dai Stablecoin"
/// );
/// ```
#[macro_export]
macro_rules! token {
    ($chain_id:expr, $address:literal, $decimals:expr) => {
        Token::new(
            $chain_id,
            alloy_primitives::address!($address),
            $decimals,
            None,
            None,
            0,
            0,
        )
    };
    ($chain_id:expr, $address:expr, $decimals:expr) => {
        Token::new(
            $chain_id,
            $address
                .to_string()
                .parse::<alloy_primitives::Address>()
                .unwrap(),
            $decimals,
            None,
            None,
            0,
            0,
        )
    };
    ($chain_id:expr, $address:literal, $decimals:expr, $symbol:expr) => {
        Token::new(
            $chain_id,
            alloy_primitives::address!($address),
            $decimals,
            Some($symbol.to_string()),
            None,
            0,
            0,
        )
    };
    ($chain_id:expr, $address:expr, $decimals:expr, $symbol:expr) => {
        Token::new(
            $chain_id,
            $address
                .to_string()
                .parse::<alloy_primitives::Address>()
                .unwrap(),
            $decimals,
            Some($symbol.to_string()),
            None,
            0,
            0,
        )
    };
    ($chain_id:expr, $address:literal, $decimals:expr, $symbol:expr, $name:expr) => {
        Token::new(
            $chain_id,
            alloy_primitives::address!($address),
            $decimals,
            Some($symbol.to_string()),
            Some($name.to_string()),
            0,
            0,
        )
    };
    ($chain_id:expr, $address:expr, $decimals:expr, $symbol:expr, $name:expr) => {
        Token::new(
            $chain_id,
            $address
                .to_string()
                .parse::<alloy_primitives::Address>()
                .unwrap(),
            $decimals,
            Some($symbol.to_string()),
            Some($name.to_string()),
            0,
            0,
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";
    const ADDRESS_TWO: &str = "0x0000000000000000000000000000000000000002";
    const DAI_MAINNET: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

    mod equals {
        use super::*;

        #[test]
        fn fails_if_address_differs() {
            assert!(!token!(1, ADDRESS_ONE, 18).equals(&token!(1, ADDRESS_TWO, 18)));
        }

        #[test]
        fn false_if_chain_id_differs() {
            assert!(!token!(3, ADDRESS_ONE, 18).equals(&token!(1, ADDRESS_ONE, 18)));
        }

        #[test]
        fn true_if_only_decimals_differs() {
            assert!(token!(1, ADDRESS_ONE, 9).equals(&token!(1, ADDRESS_ONE, 18)));
        }

        #[test]
        fn true_if_address_is_the_same() {
            assert!(token!(1, ADDRESS_ONE, 18).equals(&token!(1, ADDRESS_ONE, 18)));
        }

        #[test]
        fn true_on_reference_equality() {
            let token = token!(1, ADDRESS_ONE, 18);
            assert!(token.equals(&token));
        }

        #[test]
        fn true_even_if_name_symbol_decimals_differ() {
            let token_a = token!(1, ADDRESS_ONE, 9, "abc", "def");
            let token_b = token!(1, ADDRESS_ONE, 18, "ghi", "jkl");
            assert!(token_a.equals(&token_b));
        }

        #[test]
        fn true_even_if_one_token_is_checksummed_and_the_other_is_not() {
            let token_a = token!(1, DAI_MAINNET, 18, "DAI");
            let token_b = token!(1, &DAI_MAINNET.to_lowercase(), 18, "DAI");
            assert!(token_a.equals(&token_b));
        }
    }
}
