// External crate dependencies
use crate::prelude::*;

// Type alias for a currency amount using the FractionLike trait
pub type CurrencyAmount<T> = FractionLike<CurrencyMeta<T>>;

// Struct representing metadata about a currency
#[derive(Clone, PartialEq)]
pub struct CurrencyMeta<T: CurrencyTrait> {
    pub currency: T,
    pub decimal_scale: BigUint,
}

// Implementation of methods for CurrencyAmount
impl<T: CurrencyTrait> CurrencyAmount<T> {
    // Constructor method for creating a new currency amount
    fn new(
        currency: T,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        let numerator = numerator.into();
        let denominator = denominator.into();
        // Ensure the amount does not exceed MAX_UINT256
        if !numerator.div_floor(&denominator).le(&MAX_UINT256) {
            return Err(Error::MaxUint);
        }
        let exponent = currency.decimals();
        Ok(FractionTrait::new(
            numerator,
            denominator,
            CurrencyMeta {
                currency,
                decimal_scale: BigUint::from(10u64).pow(exponent as u32),
            },
        ))
    }

    // Returns a new currency amount instance from the unitless amount of token (raw amount)
    pub fn from_raw_amount(currency: T, raw_amount: impl Into<BigInt>) -> Result<Self, Error> {
        Self::new(currency, raw_amount, 1)
    }

    // Construct a currency amount with a denominator that is not equal to 1
    pub fn from_fractional_amount(
        currency: T,
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        Self::new(currency, numerator, denominator)
    }

    // Multiplication of currency amount by another fractional amount
    pub fn multiply<M>(&self, other: &impl FractionTrait<M>) -> Result<Self, Error> {
        let multiplied = self.as_fraction() * other.as_fraction();
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            multiplied.numerator().clone(),
            multiplied.denominator().clone(),
        )
    }

    // Division of currency amount by another fractional amount
    pub fn divide<M>(&self, other: &impl FractionTrait<M>) -> Result<Self, Error> {
        let divided = self.as_fraction() / other.as_fraction();
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            divided.numerator().clone(),
            divided.denominator().clone(),
        )
    }

    // Convert the currency amount to a string with exact precision
    pub fn to_exact(&self) -> String {
        BigDecimal::from_str(&self.quotient().to_str_radix(10))
            .unwrap()
            .div(BigDecimal::from_str(&self.meta.decimal_scale.to_str_radix(10)).unwrap())
            .to_string()
    }

    // Addition of another currency amount to the current amount
    pub fn add(&self, other: &Self) -> Result<Self, Error> {
        if !self.meta.currency.equals(&other.meta.currency) {
            return Err(Error::NotEqual("CURRENCY: not equal".to_owned()));
        }
        let added = self.as_fraction() + other.as_fraction();
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            added.numerator().clone(),
            added.denominator().clone(),
        )
    }

    // Subtraction of another currency amount from the current amount
    pub fn subtract(&self, other: &Self) -> Result<Self, Error> {
        if !self.meta.currency.equals(&other.meta.currency) {
            return Err(Error::NotEqual("CURRENCY: not equal".to_owned()));
        }
        let subtracted = self.as_fraction() - other.as_fraction();
        Self::from_fractional_amount(
            self.meta.currency.clone(),
            subtracted.numerator().clone(),
            subtracted.denominator().clone(),
        )
    }

    // Convert the currency amount to a string with a specified number of significant digits
    pub fn to_significant(
        &self,
        significant_digits: u8,
        rounding: Rounding,
    ) -> Result<String, Error> {
        (self.as_fraction() / Fraction::new(self.meta.decimal_scale.clone(), 1))
            .to_significant(significant_digits, rounding)
    }

    // Convert the currency amount to a string with a fixed number of decimal places
    pub fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> Result<String, Error> {
        if !decimal_places <= self.meta.currency.decimals() {
            return Err(Error::NotEqual(format!(
                "{} should be less than or equal to {}",
                decimal_places,
                self.meta.currency.decimals()
            )));
        }

        Ok(
            (self.as_fraction() / Fraction::new(self.meta.decimal_scale.clone(), 1))
                .to_fixed(decimal_places, rounding),
        )
    }
}

// Implementation for a specific type of CurrencyAmount (Token)
impl CurrencyAmount<Token> {
    // Wrap the currency amount if the currency is not native
    pub fn wrapped(&self) -> Result<CurrencyAmount<Token>, Error> {
        match &self.meta.currency.is_native() {
            true => Self::from_fractional_amount(
                self.meta.currency.wrapped(),
                self.numerator().clone(),
                self.denominator().clone(),
            ),
            false => Ok(self.clone()),
        }
    }
}

// Unit tests for the currency module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    // Constants for testing
    const ADDRESS_ONE: &str = "0x0000000000000000000000000000000000000001";

    // Lazy static variables for testing currencies
    lazy_static! {
        static ref TOKEN18: Currency = Currency::Token(token!(1, ADDRESS_ONE, 18));
        static ref TOKEN0: Currency = Currency::Token(token!(1, ADDRESS_ONE, 0));
    }

    // Unit tests
    #[test]
    fn test_constructor() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 100);
        assert_eq!(amount.unwrap().quotient(), 100.into());
    }

    #[test]
    fn test_quotient() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 100)
            .unwrap()
            .multiply(&Percent::new(15, 100));
        assert_eq!(amount.unwrap().quotient(), BigInt::from(15));
    }

    #[test]
    fn test_ether() {
        let ether = Ether::on_chain(1);
        let amount =
            CurrencyAmount::from_raw_amount(Currency::NativeCurrency(ether.clone()), 100).unwrap();
        assert_eq!(amount.clone().quotient(), 100.into());
        assert!(amount
            .clone()
            .meta
            .currency
            .equals(&Currency::NativeCurrency(ether.clone())));
    }

    #[test]
    fn test_token_amount_max_uint256() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), MAX_UINT256.clone());
        assert_eq!(amount.unwrap().quotient(), MAX_UINT256.clone());
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
        let amount = CurrencyAmount::from_fractional_amount(TOKEN18.clone(), numerator.clone(), 2);
        assert_eq!(amount.unwrap().numerator(), &numerator);
    }

    #[test]
    #[should_panic(expected = "DECIMALS")]
    fn to_fixed_decimals_exceeds_currency_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000);
        let _w = amount.unwrap().to_fixed(3, Rounding::RoundDown);
        assert!(_w.is_err(), "DECIMALS");
    }

    #[test]
    fn to_fixed_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456);
        assert_eq!(
            amount.unwrap().to_fixed(0, Rounding::RoundDown).unwrap(),
            "123456"
        );
    }

    #[test]
    fn to_fixed_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 1e15 as i64);
        assert_eq!(
            amount.unwrap().to_fixed(9, Rounding::RoundDown).unwrap(),
            "0.001000000"
        );
    }

    #[test]
    fn to_significant_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000);
        assert_eq!(
            amount
                .unwrap()
                .to_significant(3, Rounding::RoundDown)
                .unwrap(),
            "1000"
        );
    }

    #[test]
    fn to_significant_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456);
        assert_eq!(
            amount
                .unwrap()
                .to_significant(4, Rounding::RoundDown)
                .unwrap(),
            "123400"
        );
    }

    #[test]
    fn to_significant_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 1e15 as i64);
        assert_eq!(
            amount
                .unwrap()
                .to_significant(9, Rounding::RoundDown)
                .unwrap(),
            "0.001"
        );
    }

    #[test]
    fn to_exact_does_not_throw() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 1000);
        assert_eq!(amount.unwrap().to_exact(), "1000");
    }

    #[test]
    fn to_exact_0_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN0.clone(), 123456).unwrap();
        println!("{:?}", amount.clone().to_exact());
        assert_eq!(amount.clone().to_exact(), "123456");
    }

    #[test]
    fn to_exact_18_decimals() {
        let amount = CurrencyAmount::from_raw_amount(TOKEN18.clone(), 123e13 as i64);
        assert_eq!(amount.unwrap().to_exact(), "0.00123");
    }
}
