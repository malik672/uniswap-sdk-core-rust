use crate::prelude::*;
use core::ops::Div;

/// Currency amount struct that represents a rational amount of a currency
pub type CurrencyAmount<T> = FractionLike<CurrencyMeta<T>>;

/// Struct representing metadata about a currency
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct CurrencyMeta<T: BaseCurrency> {
    /// The currency associated with this metadata
    pub currency: T,
    /// The scale factor for the currency's decimal places
    pub decimal_scale: BigUint,
}

impl<T: BaseCurrency> CurrencyAmount<T> {
    /// Constructor method for creating a new currency amount
    #[inline]
    fn new(
        currency: T,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        let numerator = numerator.into();
        let denominator = denominator.into();
        // Ensure the amount does not exceed MAX_UINT256
        if numerator.div_floor(&denominator) > *MAX_UINT256 {
            return Err(Error::UintOverflow);
        }
        let exponent = currency.decimals();
        Ok(FractionBase::new(
            numerator,
            denominator,
            CurrencyMeta {
                currency,
                decimal_scale: BigUint::from(10_u64).pow(exponent as u32),
            },
        ))
    }

    /// Returns a new currency amount instance from the unitless amount of token (raw amount)
    #[inline]
    pub fn from_raw_amount(currency: T, raw_amount: impl Into<BigInt>) -> Result<Self, Error> {
        Self::new(currency, raw_amount, 1)
    }

    /// Construct a currency amount with a denominator that is not equal to 0
    #[inline]
    pub fn from_fractional_amount(
        currency: T,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        Self::new(currency, numerator, denominator)
    }

    /// Multiplication of currency amount by another fractional amount
    #[inline]
    pub fn multiply<M: Clone>(&self, other: &impl FractionBase<M>) -> Result<Self, Error> {
        let multiplied = self.as_fraction() * other.as_fraction();
        Self::from_fractional_amount(
            self.currency.clone(),
            multiplied.numerator,
            multiplied.denominator,
        )
    }

    /// Division of currency amount by another fractional amount
    #[inline]
    pub fn divide<M: Clone>(&self, other: &impl FractionBase<M>) -> Result<Self, Error> {
        let divided = self.as_fraction() / other.as_fraction();
        Self::from_fractional_amount(
            self.currency.clone(),
            divided.numerator,
            divided.denominator,
        )
    }

    /// Convert the currency amount to a string with exact precision
    #[inline]
    pub fn to_exact(&self) -> String {
        BigDecimal::from(self.quotient())
            .div(BigDecimal::from(BigInt::from(self.decimal_scale.clone())))
            .to_string()
    }

    /// Addition of another currency amount to the current amount
    #[inline]
    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        if !self.currency.equals(&other.currency) {
            return Err(Error::CurrencyMismatch);
        }
        let added = self.as_fraction() + other.as_fraction();
        Self::from_fractional_amount(self.currency.clone(), added.numerator, added.denominator)
    }

    /// Subtraction of another currency amount from the current amount
    #[inline]
    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        if !self.currency.equals(&other.currency) {
            return Err(Error::CurrencyMismatch);
        }
        let subtracted = self.as_fraction() - other.as_fraction();
        Self::from_fractional_amount(
            self.currency.clone(),
            subtracted.numerator,
            subtracted.denominator,
        )
    }

    /// Convert the currency amount to a string with a specified number of significant digits
    #[inline]
    pub fn to_significant(
        &self,
        significant_digits: u8,
        rounding: Rounding,
    ) -> Result<String, Error> {
        (self.as_fraction() / Fraction::new(self.decimal_scale.clone(), 1))
            .to_significant(significant_digits, rounding)
    }

    /// Convert the currency amount to a string with a fixed number of decimal places
    #[inline]
    pub fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> Result<String, Error> {
        if decimal_places > self.currency.decimals() {
            return Err(Error::Invalid("DECIMALS"));
        }

        if decimal_places == 0 {
            // Directly convert the numerator to a string for zero decimal places
            return Ok(self.numerator().to_string());
        }

        Ok(
            (self.as_fraction() / Fraction::new(self.decimal_scale.clone(), 1))
                .to_fixed(decimal_places, rounding),
        )
    }

    /// Wrap the currency amount if the currency is not native
    #[inline]
    pub fn wrapped(&self) -> Result<CurrencyAmount<&Token>, Error> {
        CurrencyAmount::from_fractional_amount(
            self.currency.wrapped(),
            self.numerator().clone(),
            self.denominator().clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    // Constants for testing
    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    // Lazy static variables for testing currencies
    lazy_static! {
        static ref TOKEN18: Token = token!(1, ADDRESS_ONE, 18);
        static ref TOKEN0: Token = token!(1, ADDRESS_ONE, 0);
    }

    // Unit tests
    #[test]
    fn test_constructor() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 100).unwrap();
        assert_eq!(amount.quotient(), 100.into());
    }

    #[test]
    fn test_quotient() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 100)
            .unwrap()
            .multiply(&Percent::new(15, 100))
            .unwrap();
        assert_eq!(amount.quotient(), BigInt::from(15));
    }

    #[test]
    fn test_ether() {
        let ether = Ether::on_chain(1);
        let amount = CurrencyAmount::from_raw_amount(ether.clone(), 100).unwrap();
        assert_eq!(amount.quotient(), 100.into());
        assert!(amount.currency.equals(&ether));
    }

    #[test]
    fn test_token_amount_max_uint256() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), MAX_UINT256.clone()).unwrap();
        assert_eq!(amount.quotient(), MAX_UINT256.clone());
    }

    #[test]
    #[should_panic(expected = "AMOUNT")]
    fn test_token_amount_exceeds_max_uint256() {
        let _w = CurrencyAmount::from_raw_amount(TOKEN18.clone(), MAX_UINT256.clone() + 1);
        assert!(_w.is_ok(), "AMOUNT");
    }

    #[test]
    #[should_panic(expected = "AMOUNT")]
    fn test_token_amount_quotient_exceeds_max_uint256() {
        let numerator: BigInt = (MAX_UINT256.clone() + 1) * 2;
        let _w = CurrencyAmount::from_fractional_amount(TOKEN18.clone(), numerator, 2);
        assert!(_w.is_ok(), "AMOUNT");
    }

    #[test]
    fn test_token_amount_numerator_gt_uint256() {
        let numerator: BigInt = MAX_UINT256.clone() + 2;
        let amount =
            CurrencyAmount::from_fractional_amount(TOKEN18.clone(), numerator.clone(), 2).unwrap();
        assert_eq!(amount.numerator(), &numerator);
    }

    #[test]
    fn to_fixed_decimals_exceeds_currency_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000).unwrap();
        let _w = amount.to_fixed(3, Rounding::RoundDown);
        assert!(_w.is_err(), "DECIMALS");
    }

    #[test]
    fn to_fixed_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456).unwrap();
        assert_eq!(amount.to_fixed(0, Rounding::RoundDown).unwrap(), "123456");
    }

    #[test]
    fn to_fixed_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 1e15 as i64).unwrap();
        assert_eq!(
            amount.to_fixed(9, Rounding::RoundDown).unwrap(),
            "0.001000000"
        );
    }

    #[test]
    fn to_significant_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000).unwrap();
        assert_eq!(
            amount.to_significant(3, Rounding::RoundDown).unwrap(),
            "1000"
        );
    }

    #[test]
    fn to_significant_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456).unwrap();
        assert_eq!(
            amount.to_significant(4, Rounding::RoundDown).unwrap(),
            "123400"
        );
    }

    #[test]
    fn to_significant_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 1e15 as i64).unwrap();
        assert_eq!(
            amount.to_significant(9, Rounding::RoundDown).unwrap(),
            "0.001"
        );
    }

    #[test]
    fn to_exact_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000).unwrap();
        assert_eq!(amount.to_exact(), "1000");
    }

    #[test]
    fn to_exact_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456).unwrap();
        assert_eq!(amount.to_exact(), "123456");
    }

    #[test]
    fn to_exact_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 123e13 as i64).unwrap();
        assert_eq!(amount.to_exact(), "0.00123");
    }
}
