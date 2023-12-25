use crate::{
    constants::Rounding,
    entities::{
        currency::CurrencyTrait,
        fractions::{
            currency_amount::CurrencyAmount,
            fraction::{Fraction, FractionTrait},
        },
    },
};
use num_bigint::BigInt;

#[derive(Clone)]
pub struct Price<TBase, TQuote>
where
    TBase: CurrencyTrait,
    TQuote: CurrencyTrait,
{
    numerator: BigInt,
    denominator: BigInt,
    pub meta: PriceMeta<TBase, TQuote>,
}

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

impl<TBase, TQuote> FractionTrait<PriceMeta<TBase, TQuote>> for Price<TBase, TQuote>
where
    TBase: CurrencyTrait,
    TQuote: CurrencyTrait,
{
    fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
        meta: PriceMeta<TBase, TQuote>,
    ) -> Self {
        Self {
            numerator: numerator.into(),
            denominator: denominator.into(),
            meta,
        }
    }

    fn meta(&self) -> PriceMeta<TBase, TQuote> {
        self.meta.clone()
    }

    fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    fn denominator(&self) -> &BigInt {
        &self.denominator
    }

    fn to_significant(&self, significant_digits: u32, rounding: Rounding) -> String {
        self.adjusted_for_decimals()
            .to_significant(significant_digits, rounding)
    }

    fn to_fixed(&self, decimal_places: u32, rounding: Rounding) -> String {
        self.adjusted_for_decimals()
            .to_fixed(decimal_places, rounding)
    }
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
            BigInt::from(10).pow(base_currency.decimals()),
            BigInt::from(10).pow(quote_currency.decimals()),
            (),
        );
        Self {
            numerator: numerator.into(),
            denominator: denominator.into(),
            meta: PriceMeta {
                base_currency,
                quote_currency,
                scalar,
            },
        }
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
    pub fn invert(self) -> Price<TQuote, TBase> {
        Price::new(
            self.meta.quote_currency,
            self.meta.base_currency,
            self.numerator,
            self.denominator,
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
        other: Price<TQuote, TOtherQuote>,
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
}
