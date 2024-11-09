use crate::prelude::*;

/// Type alias for a Price, a [`FractionLike`] with metadata [`PriceMeta`]
pub type Price<TBase, TQuote> = FractionLike<PriceMeta<TBase, TQuote>>;

/// Struct representing metadata for a [`Price`]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct PriceMeta<TBase, TQuote>
where
    TBase: BaseCurrency,
    TQuote: BaseCurrency,
{
    /// The base currency for the price
    pub base_currency: TBase,

    /// The quote currency for the price
    pub quote_currency: TQuote,

    /// The scalar used to adjust the price for decimal places
    pub scalar: Fraction,
}

impl<TBase, TQuote> Price<TBase, TQuote>
where
    TBase: BaseCurrency,
    TQuote: BaseCurrency,
{
    /// Constructor for creating a new [`Price`] instance
    #[inline]
    pub fn new(
        base_currency: TBase,
        quote_currency: TQuote,
        denominator: impl Into<BigInt>,
        numerator: impl Into<BigInt>,
    ) -> Self {
        // Calculate scalar based on decimal places of base and quote currencies
        let scalar = Fraction::new(
            BigInt::from(10).pow(base_currency.decimals() as u32),
            BigInt::from(10).pow(quote_currency.decimals() as u32),
        );
        FractionBase::new(
            numerator,
            denominator,
            PriceMeta {
                base_currency,
                quote_currency,
                scalar,
            },
        )
    }

    /// Create a [`Price`] instance from currency amounts of the base and quote currencies
    #[inline]
    pub fn from_currency_amounts(
        base_amount: CurrencyAmount<TBase>,
        quote_amount: CurrencyAmount<TQuote>,
    ) -> Self {
        let res = quote_amount.divide(&base_amount).unwrap();
        Self::new(
            base_amount.meta.currency,
            quote_amount.meta.currency,
            res.denominator,
            res.numerator,
        )
    }

    /// Flip the price, switching the base and quote currency
    #[inline]
    pub fn invert(&self) -> Price<TQuote, TBase> {
        Price::new(
            self.quote_currency.clone(),
            self.base_currency.clone(),
            self.numerator.clone(),
            self.denominator.clone(),
        )
    }

    /// Multiply the price by another price, returning a new price.
    /// The other price must have the same base currency as this price's quote currency.
    #[inline]
    pub fn multiply<TOtherQuote: BaseCurrency>(
        &self,
        other: &Price<TQuote, TOtherQuote>,
    ) -> Result<Price<TBase, TOtherQuote>, Error> {
        if !self.quote_currency.equals(&other.base_currency) {
            return Err(Error::CurrencyMismatch);
        }
        let fraction = self.as_fraction() * other.as_fraction();
        Ok(Price::new(
            self.base_currency.clone(),
            other.quote_currency.clone(),
            fraction.denominator,
            fraction.numerator,
        ))
    }

    /// Return the amount of quote currency corresponding to a given amount of the base currency
    #[inline]
    pub fn quote(
        &self,
        currency_amount: &CurrencyAmount<TBase>,
    ) -> Result<CurrencyAmount<TQuote>, Error> {
        if !currency_amount.currency.equals(&self.base_currency) {
            return Err(Error::CurrencyMismatch);
        }
        let fraction = self.as_fraction() * currency_amount.as_fraction();
        CurrencyAmount::from_fractional_amount(
            self.quote_currency.clone(),
            fraction.numerator,
            fraction.denominator,
        )
    }

    /// Get the value scaled by decimals for formatting
    #[inline]
    pub fn adjusted_for_decimals(&self) -> Fraction {
        self.as_fraction() * &self.scalar
    }

    /// Converts the adjusted price to a string with a specified number of significant digits and
    /// rounding strategy
    #[inline]
    pub fn to_significant(
        &self,
        significant_digits: u8,
        rounding: Option<Rounding>,
    ) -> Result<String, Error> {
        self.adjusted_for_decimals()
            .to_significant(significant_digits, rounding)
    }

    /// Converts the adjusted price to a string with a fixed number of decimal places and rounding
    /// strategy
    #[inline]
    pub fn to_fixed(&self, decimal_places: u8, rounding: Option<Rounding>) -> String {
        self.adjusted_for_decimals()
            .to_fixed(decimal_places, rounding)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token;

    const ADDRESS_ZERO: &str = "0x0000000000000000000000000000000000000000";
    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    lazy_static! {
        static ref TOKEN0: Token = token!(1, ADDRESS_ZERO, 18);
        static ref TOKEN0_6: Token = token!(1, ADDRESS_ZERO, 6);
        static ref TOKEN1: Token = token!(1, ADDRESS_ONE, 18);
    }

    mod constructor {
        use super::*;

        #[test]
        fn array_format_works() {
            let price = Price::new(TOKEN0.clone(), TOKEN1.clone(), 1, 54321);
            assert_eq!(price.to_significant(5, None).unwrap(), "54321");
            assert!(price.base_currency.equals(&TOKEN0.clone()));
            assert!(price.quote_currency.equals(&TOKEN1.clone()));
        }

        #[test]
        fn object_format_works() {
            let price = Price::from_currency_amounts(
                CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1).unwrap(),
                CurrencyAmount::from_raw_amount(TOKEN1.clone(), 54321).unwrap(),
            );
            assert_eq!(price.to_significant(5, None).unwrap(), "54321");
            assert!(price.base_currency.equals(&TOKEN0.clone()));
            assert!(price.quote_currency.equals(&TOKEN1.clone()));
        }
    }

    #[test]
    fn test_quote_returns_correct_value() {
        let price = Price::new(TOKEN0.clone(), TOKEN1.clone(), 1, 5);
        assert_eq!(
            price
                .quote(&CurrencyAmount::from_raw_amount(TOKEN0.clone(), 10).unwrap())
                .unwrap(),
            CurrencyAmount::from_raw_amount(TOKEN1.clone(), 50).unwrap()
        );
    }

    mod to_significant {
        use super::*;

        #[test]
        fn no_decimals() {
            let p = Price::new(TOKEN0.clone(), TOKEN1.clone(), 123, 456);
            assert_eq!(p.to_significant(4, None).unwrap(), "3.707");
        }

        #[test]
        fn no_decimals_flip_ratio() {
            let p = Price::new(TOKEN0.clone(), TOKEN1.clone(), 456, 123);
            assert_eq!(p.to_significant(4, None).unwrap(), "0.2697");
        }

        #[test]
        fn with_decimal_difference() {
            let p = Price::new(TOKEN0_6.clone(), TOKEN1.clone(), 123, 456);
            assert_eq!(p.to_significant(4, None).unwrap(), "3.707E-12");
        }

        #[test]
        fn with_decimal_difference_flipped() {
            let p = Price::new(TOKEN0_6.clone(), TOKEN1.clone(), 456, 123);
            assert_eq!(p.to_significant(4, None).unwrap(), "2.697E-13");
        }

        #[test]
        fn with_decimal_difference_flipped_base_quote_flipped() {
            let p = Price::new(TOKEN1.clone(), TOKEN0_6.clone(), 456, 123);
            assert_eq!(p.to_significant(4, None).unwrap(), "269700000000");
        }
    }
}
