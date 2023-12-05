

use crate::entities::{currency::Currency, fractions::fraction::Fraction};
use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;

#[derive(Clone, PartialEq)]
pub struct CurrencyAmount {
    pub currency: Currency,
    pub decimal_scale: Option<BigUint>,
    pub fraction: Option<Fraction>,
}

impl CurrencyAmount {
    pub fn new(
        currency: Currency,
        decimal_scale: Option<BigUint>,
        fraction: Option<Fraction>,
    ) -> Self {
        Self {
            currency,
            decimal_scale,
            fraction,
        }
    }

    pub fn from_raw_amount(currency: Currency, raw_amount: BigInt) -> CurrencyAmount {
        let decimal_scale = BigUint::from(10_u32.pow(18_u32));
        let fraction = Fraction {
            numerator: raw_amount.clone(),
            denominator: BigInt::from(1),
        };

        Self {
            currency,
            decimal_scale: Some(decimal_scale),
            fraction: Some(fraction),
        }
    }

    pub fn from_fractional_amount(
        currency: Currency,
        numerator: BigInt,
        denominator: BigInt,
    ) -> CurrencyAmount {
        CurrencyAmount::new(currency, None, Some(Fraction::new(numerator, denominator)))
    }

    pub fn add(&self, other: CurrencyAmount) -> CurrencyAmount {
        assert!(self.currency == other.currency, "CURRENCY SHOULD EQUAL");
        let add = Fraction::add(&self.fraction.clone().unwrap(), &other.fraction.unwrap());
        CurrencyAmount::from_fractional_amount(other.currency, add.numerator, add.denominator)
    }

    pub fn subtract(&self, other: CurrencyAmount) -> CurrencyAmount {
        assert!(self.currency == other.currency, "CURRENCY SHOULD EQUAL");
        let sub = Fraction::subtract(&self.fraction.clone().unwrap(), &other.fraction.unwrap());
        CurrencyAmount::from_fractional_amount(other.currency, sub.numerator, sub.denominator)
    }

    pub fn multiply(&self, other: CurrencyAmount) -> CurrencyAmount {
        let multiplied =
            Fraction::multiply(&self.fraction.clone().unwrap(), &other.fraction.unwrap());
        CurrencyAmount::from_fractional_amount(
            other.currency,
            multiplied.numerator,
            multiplied.denominator,
        )
    }

    pub fn divide(&self, other: CurrencyAmount) -> CurrencyAmount {
        let divided = Fraction::divide(&self.fraction.clone().unwrap(), &other.fraction.unwrap());
        CurrencyAmount::from_fractional_amount(
            other.currency,
            divided.numerator,
            divided.denominator,
        )
    }

    pub fn to_significant(&self) -> String {
        Fraction::to_significant(&self.fraction.clone().unwrap(), 6)
    }

    pub fn to_fixed(&self, decimals: BigUint) -> String {
        assert!(decimals <= self.decimal_scale.clone().unwrap(), "DECIMALS");
        Fraction::to_fixed(&self.fraction.clone().unwrap(), decimals.to_u32().unwrap())
    }
    
    //Implementation not done yet
    pub fn to_exact(){}

    // pub fn wrapped(&self) -> CurrencyAmount {
    //   if Token::is_token() {//don't really understand this part, but is_token will always return true anyway
    //     self.clone()
    //   } else {
    //     let wrapped = Token::wrapped(Currency::Token(Token));
    //   }
    // }
}
