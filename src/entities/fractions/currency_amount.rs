use crate::{
    constants::{Rounding, MAX_UINT256},
    entities::{
        base_currency::BaseCurrency,
        currency::Currency,
        fractions::fraction::{Fraction, FractionTrait},
    },
};
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_integer::Integer;
use rust_decimal::Decimal;
use std::{ops::Div, str::FromStr};

#[derive(Clone)]
pub struct CurrencyAmount<'a> {
    numerator: BigInt,
    denominator: BigInt,
    pub meta: CurrencyMeta<'a>,
}

#[derive(Clone)]
pub struct CurrencyMeta<'a> {
    pub currency: Currency<'a>,
    pub decimal_scale: BigUint,
}

impl<'a> CurrencyAmount<'a> {
    fn new(
        currency: Currency<'a>,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Self {
        let numerator = numerator.into();
        let denominator = denominator.into();
        assert!(numerator.div_floor(&denominator).le(&MAX_UINT256), "AMOUNT");
        let exponent = match &currency {
            Currency::Token(currency) => currency.decimals(),
            Currency::NativeCurrency(currency) => currency.decimals(),
        };
        Self {
            numerator,
            denominator,
            meta: CurrencyMeta {
                currency,
                decimal_scale: BigUint::from(10u64).pow(exponent),
            },
        }
    }

    /// Returns a new currency amount instance from the unitless amount of token, i.e. the raw amount
    ///
    /// # Arguments
    ///
    /// * `currency`: the currency in the amount
    /// * `raw_amount`: the raw token or ether amount
    ///
    /// returns: CurrencyAmount
    ///
    pub fn from_raw_amount(
        currency: Currency<'a>,
        raw_amount: impl Into<BigInt>,
    ) -> CurrencyAmount {
        Self::new(currency, raw_amount, 1)
    }

    /// Construct a currency amount with a denominator that is not equal to 1
    ///
    /// # Arguments
    ///
    /// * `currency`: the currency
    /// * `numerator`: the numerator of the fractional token amount
    /// * `denominator`: the denominator of the fractional token amount
    ///
    /// returns: CurrencyAmount
    ///
    pub fn from_fractional_amount(
        currency: Currency<'a>,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> CurrencyAmount {
        Self::new(currency, numerator, denominator)
    }

    pub fn multiply<M>(&self, other: impl FractionTrait<M>) -> Self {
        let multiplied = self.as_fraction().multiply(&other.as_fraction());
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            multiplied.numerator().clone(),
            multiplied.denominator().clone(),
        )
    }

    pub fn divide<M>(&self, other: impl FractionTrait<M>) -> Self {
        let divided = self.as_fraction().divide(&other.as_fraction());
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            divided.numerator().clone(),
            divided.denominator().clone(),
        )
    }

    pub fn to_exact(&self) -> String {
        Decimal::from_str(&self.quotient().to_str_radix(10))
            .unwrap()
            .div(Decimal::from_str(&self.meta.decimal_scale.to_str_radix(10)).unwrap())
            .to_string()
    }

    pub fn wrapped(&self) -> CurrencyAmount {
        match &self.meta.currency {
            Currency::NativeCurrency(native_currency) => Self::from_fractional_amount(
                Currency::Token(native_currency.wrapped()),
                self.numerator.clone(),
                self.denominator.clone(),
            ),
            Currency::Token(_) => self.clone(),
        }
    }
}

impl<'a> FractionTrait<CurrencyMeta<'a>> for CurrencyAmount<'a> {
    fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
        meta: CurrencyMeta<'a>,
    ) -> Self {
        Self {
            numerator: numerator.into(),
            denominator: denominator.into(),
            meta,
        }
    }

    fn meta(&self) -> CurrencyMeta<'a> {
        self.meta.clone()
    }

    fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    fn add(&self, other: &Self) -> Self {
        assert!(self.meta.currency.equals(&other.meta.currency), "CURRENCY");
        let added = self.as_fraction().add(&other.as_fraction());
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            added.numerator().clone(),
            added.denominator().clone(),
        )
    }

    fn subtract(&self, other: &Self) -> Self {
        assert!(self.meta.currency.equals(&other.meta.currency), "CURRENCY");
        let subtracted = self.as_fraction().subtract(&other.as_fraction());
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            subtracted.numerator().clone(),
            subtracted.denominator().clone(),
        )
    }

    fn to_significant(&self, significant_digits: u32, rounding: Rounding) -> String {
        self.as_fraction()
            .divide(&Fraction::new(
                self.meta.decimal_scale.to_bigint().unwrap(),
                1,
                (),
            ))
            .to_significant(significant_digits, rounding)
    }

    fn to_fixed(&self, decimal_places: u32, rounding: Rounding) -> String {
        assert!(decimal_places <= self.meta.currency.decimals(), "DECIMALS");
        self.as_fraction()
            .divide(&Fraction::new(
                self.meta.decimal_scale.to_bigint().unwrap(),
                1,
                (),
            ))
            .to_fixed(decimal_places, rounding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{ether::Ether, fractions::percent::Percent, token::Token};

    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    fn token18() -> Currency<'static> {
        Currency::Token(Token::new(
            1,
            ADDRESS_ONE.to_string(),
            18,
            None,
            None,
            None,
            None,
        ))
    }

    fn token0() -> Currency<'static> {
        Currency::Token(Token::new(
            1,
            ADDRESS_ONE.to_string(),
            0,
            None,
            None,
            None,
            None,
        ))
    }

    #[test]
    fn test_constructor() {
        let amount = CurrencyAmount::from_raw_amount(token18(), 100);
        assert_eq!(amount.quotient(), 100.into());
    }

    #[test]
    fn test_quotient() {
        let amount =
            CurrencyAmount::from_raw_amount(token18(), 100).multiply(Percent::new(15, 100, ()));
        assert_eq!(amount.quotient(), BigInt::from(15));
    }

    #[test]
    fn test_ether() {
        let ether = Ether::on_chain(1);
        let amount = CurrencyAmount::from_raw_amount(Currency::NativeCurrency(&ether), 100);
        assert_eq!(amount.quotient(), 100.into());
        assert!(amount
            .meta
            .currency
            .equals(&Currency::NativeCurrency(&ether)));
    }

    #[test]
    fn test_token_amount_max_uint256() {
        let amount = CurrencyAmount::from_raw_amount(token18(), MAX_UINT256.clone());
        assert_eq!(amount.quotient(), MAX_UINT256.clone());
    }

    #[test]
    #[should_panic(expected = "AMOUNT")]
    fn test_token_amount_exceeds_max_uint256() {
        let _ = CurrencyAmount::from_raw_amount(token18(), MAX_UINT256.clone() + 1);
    }

    #[test]
    #[should_panic(expected = "AMOUNT")]
    fn test_token_amount_quotient_exceeds_max_uint256() {
        let numerator: BigInt = (MAX_UINT256.clone() + 1) * 2;
        let _ = CurrencyAmount::from_fractional_amount(token18(), numerator, 2);
    }

    #[test]
    fn test_token_amount_numerator_gt_uint256() {
        let numerator: BigInt = MAX_UINT256.clone() + 2;
        let amount = CurrencyAmount::from_fractional_amount(token18(), numerator.clone(), 2);
        assert_eq!(amount.numerator(), &numerator);
    }

    #[test]
    #[should_panic(expected = "DECIMALS")]
    fn to_fixed_decimals_exceeds_currency_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 1000);
        let _ = amount.to_fixed(3, Rounding::RoundDown);
    }

    #[test]
    fn to_fixed_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 123456);
        assert_eq!(amount.to_fixed(0, Rounding::RoundDown), "123456");
    }

    #[test]
    fn to_fixed_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token18(), 1e15 as i64);
        assert_eq!(amount.to_fixed(9, Rounding::RoundDown), "0.001000000");
    }

    #[test]
    fn to_significant_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 1000);
        assert_eq!(amount.to_significant(3, Rounding::RoundDown), "1000");
    }

    #[test]
    fn to_significant_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 123456);
        assert_eq!(amount.to_significant(4, Rounding::RoundDown), "123400");
    }

    #[test]
    fn to_significant_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token18(), 1e15 as i64);
        assert_eq!(amount.to_significant(9, Rounding::RoundDown), "0.001");
    }

    #[test]
    fn to_exact_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 1000);
        assert_eq!(amount.to_exact(), "1000");
    }

    #[test]
    fn to_exact_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token0(), 123456);
        assert_eq!(amount.to_exact(), "123456");
    }

    #[test]
    fn to_exact_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(token18(), 123e13 as i64);
        assert_eq!(amount.to_exact(), "0.00123");
    }
}
