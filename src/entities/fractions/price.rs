use crate::{
    constants::Rounding,
    entities::{
        currency::CurrencyTrait,
        fractions::{
            currency_amount::CurrencyAmount,
            fraction::{Fraction, FractionLike, FractionTrait},
        },
    },
};
use num_bigint::BigInt;

pub type Price<TBase, TQuote> = FractionLike<PriceMeta<TBase, TQuote>>;

#[derive(Clone)]
pub struct PriceMeta<TBase, TQuote>
where
    TBase: CurrencyTrait,
    TQuote: CurrencyTrait,
{
    pub base_currency: TBase,
    pub quote_currency: TQuote,
    pub scalar: Fraction,
}

impl<TBase, TQuote> Price<TBase, TQuote>
where
    TBase: CurrencyTrait,
    TQuote: CurrencyTrait,
{
    pub fn new(
        base_currency: TBase,
        quote_currency: TQuote,
        denominator: impl Into<BigInt>,
        numerator: impl Into<BigInt>,
    ) -> Self {
        let scalar = Fraction::new(
            BigInt::from(10).pow(base_currency.decimals() as u32),
            BigInt::from(10).pow(quote_currency.decimals() as u32),
        );
        FractionTrait::new(
            numerator,
            denominator,
            PriceMeta {
                base_currency,
                quote_currency,
                scalar,
            },
        )
    }

    pub fn from_currency_amounts(
        base_amount: CurrencyAmount<TBase>,
        quote_amount: CurrencyAmount<TQuote>,
    ) -> Self {
        let res = quote_amount.divide(&base_amount);
        Self::new(
            base_amount.meta.currency,
            quote_amount.meta.currency,
            res.denominator().clone(),
            res.numerator().clone(),
        )
    }

    /// Flip the price, switching the base and quote currency
    pub fn invert(&self) -> Price<TQuote, TBase> {
        Price::new(
            self.meta.quote_currency.clone(),
            self.meta.base_currency.clone(),
            self.numerator().clone(),
            self.denominator().clone(),
        )
    }

    /// Multiply the price by another price, returning a new price. The other price must have the same base currency as this price's quote currency
    ///
    /// # Arguments
    ///
    /// * `other`: the other price
    ///
    /// returns: Price<TBase, TOtherQuote>
    ///
    pub fn multiply<TOtherQuote: CurrencyTrait>(
        &self,
        other: &Price<TQuote, TOtherQuote>,
    ) -> Price<TBase, TOtherQuote> {
        assert!(
            self.meta.quote_currency.equals(&other.meta.base_currency),
            "TOKEN"
        );
        let fraction = self.as_fraction().multiply(&other.as_fraction());
        Price::new(
            self.meta.base_currency.clone(),
            other.meta.quote_currency.clone(),
            fraction.denominator().clone(),
            fraction.numerator().clone(),
        )
    }

    /// Return the amount of quote currency corresponding to a given amount of the base currency
    ///
    /// # Arguments
    ///
    /// * `currency_amount`: the amount of base currency to quote against the price
    ///
    /// returns: CurrencyAmount
    ///
    pub fn quote(&self, currency_amount: CurrencyAmount<TBase>) -> CurrencyAmount<TQuote> {
        assert!(
            currency_amount
                .meta
                .currency
                .equals(&self.meta.base_currency),
            "TOKEN"
        );
        let fraction = self.as_fraction().multiply(&currency_amount.as_fraction());
        CurrencyAmount::from_fractional_amount(
            self.meta.quote_currency.clone(),
            fraction.numerator().clone(),
            fraction.denominator().clone(),
        )
    }

    /// Get the value scaled by decimals for formatting
    pub fn adjusted_for_decimals(&self) -> Fraction {
        self.as_fraction().multiply(&self.meta.scalar)
    }

    pub fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> String {
        self.adjusted_for_decimals()
            .to_significant(significant_digits, rounding)
    }

    pub fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        self.adjusted_for_decimals()
            .to_fixed(decimal_places, rounding)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entities::{base_currency::BaseCurrency, currency::Currency, token::Token};
    use lazy_static::lazy_static;

    const ADDRESS_ZERO: &str = "0x0000000000000000000000000000000000000000";
    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    lazy_static! {
        static ref TOKEN0: Currency = Currency::Token(Token::new(
            1,
            ADDRESS_ZERO.to_string(),
            18,
            None,
            None,
            None,
            None,
        ));
        static ref TOKEN0_6: Currency = Currency::Token(Token::new(
            1,
            ADDRESS_ZERO.to_string(),
            6,
            None,
            None,
            None,
            None,
        ));
        static ref TOKEN1: Currency = Currency::Token(Token::new(
            1,
            ADDRESS_ONE.to_string(),
            18,
            None,
            None,
            None,
            None,
        ));
    }

    #[test]
    fn test_constructor_array_format_works() {
        let price = Price::new(TOKEN0.clone(), TOKEN1.clone(), 1, 54321);
        assert_eq!(price.to_significant(5, Rounding::RoundDown), "54321");
        assert!(price.meta.base_currency.equals(&TOKEN0.clone()));
        assert!(price.meta.quote_currency.equals(&TOKEN1.clone()));
    }

    #[test]
    fn test_constructor_object_format_works() {
        let price = Price::from_currency_amounts(
            CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1),
            CurrencyAmount::from_raw_amount(TOKEN1.clone(), 54321),
        );
        assert_eq!(price.to_significant(5, Rounding::RoundDown), "54321");
        assert!(price.meta.base_currency.equals(&TOKEN0.clone()));
        assert!(price.meta.quote_currency.equals(&TOKEN1.clone()));
    }

    #[test]
    fn test_quote_returns_correct_value() {
        let price = Price::new(TOKEN0.clone(), TOKEN1.clone(), 1, 5);
        assert!(price
            .quote(CurrencyAmount::from_raw_amount(TOKEN0.clone(), 10))
            .equal_to(&CurrencyAmount::from_raw_amount(TOKEN1.clone(), 50)));
    }

    #[test]
    fn test_to_significant_no_decimals() {
        let p = Price::new(TOKEN0.clone(), TOKEN1.clone(), 123, 456);
        assert_eq!(p.to_significant(4, Rounding::RoundDown), "3.707");
    }

    #[test]
    fn test_to_significant_no_decimals_flip_ratio() {
        let p = Price::new(TOKEN0.clone(), TOKEN1.clone(), 456, 123);
        assert_eq!(p.to_significant(4, Rounding::RoundDown), "0.2697");
    }

    #[test]
    fn test_to_significant_with_decimal_difference() {
        let p = Price::new(TOKEN0_6.clone(), TOKEN1.clone(), 123, 456);
        assert_eq!(
            p.to_significant(4, Rounding::RoundDown),
            "0.000000000003707"
        );
    }

    #[test]
    fn test_to_significant_with_decimal_difference_flipped() {
        let p = Price::new(TOKEN0_6.clone(), TOKEN1.clone(), 456, 123);
        assert_eq!(
            p.to_significant(4, Rounding::RoundDown),
            "0.0000000000002697"
        );
    }

    #[test]
    fn test_to_significant_with_decimal_difference_flipped_base_quote_flipped() {
        let p = Price::new(TOKEN1.clone(), TOKEN0_6.clone(), 456, 123);
        assert_eq!(p.to_significant(4, Rounding::RoundDown), "269700000000");
    }
}
