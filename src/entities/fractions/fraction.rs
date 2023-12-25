use crate::constants::Rounding;
use num_bigint::BigInt;
use num_integer::Integer;
use rust_decimal::prelude::*;
use std::ops::Div;

#[derive(Clone, PartialEq)]
pub struct Fraction {
    numerator: BigInt,
    denominator: BigInt,
}

fn to_rounding_strategy(rounding: Rounding) -> RoundingStrategy {
    match rounding {
        Rounding::RoundDown => RoundingStrategy::ToZero,
        Rounding::RoundHalfUp => RoundingStrategy::MidpointAwayFromZero,
        Rounding::RoundUp => RoundingStrategy::AwayFromZero,
    }
}

pub trait FractionTrait<M>: Sized {
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self;

    /// Additional metadata that can be attached to the fraction
    fn meta(&self) -> M;

    fn numerator(&self) -> &BigInt;

    fn denominator(&self) -> &BigInt;

    /// performs floor division
    fn quotient(&self) -> BigInt {
        self.numerator().div_floor(self.denominator())
    }

    /// remainder after floor division
    fn remainder(&self) -> Self {
        Self::new(
            self.numerator() % self.denominator(),
            self.denominator().clone(),
            self.meta(),
        )
    }

    fn invert(&self) -> Self {
        Self::new(
            self.denominator().clone(),
            self.numerator().clone(),
            self.meta(),
        )
    }

    fn add(&self, other: &Self) -> Self {
        if self.denominator() == other.denominator() {
            Self::new(
                self.numerator() + other.numerator(),
                self.denominator().clone(),
                self.meta(),
            )
        } else {
            Self::new(
                self.numerator() * other.denominator() + other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }

    fn subtract(&self, other: &Self) -> Self {
        if self.denominator() == other.denominator() {
            Self::new(
                self.numerator() - other.numerator(),
                self.denominator().clone(),
                self.meta(),
            )
        } else {
            Self::new(
                self.numerator() * other.denominator() - other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }

    fn multiply(&self, other: &Self) -> Self {
        Self::new(
            self.numerator() * other.numerator(),
            self.denominator() * other.denominator(),
            self.meta(),
        )
    }

    fn divide(&self, other: &Self) -> Self {
        Self::new(
            self.numerator() * other.denominator(),
            self.denominator() * other.numerator(),
            self.meta(),
        )
    }

    fn less_than(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() < other.numerator() * self.denominator()
    }

    fn equal_to(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() == other.numerator() * self.denominator()
    }

    fn greater_than(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() > other.numerator() * self.denominator()
    }

    fn to_decimal(&self) -> Decimal {
        Decimal::from_str(&self.numerator().to_str_radix(10))
            .unwrap()
            .div(Decimal::from_str(&self.denominator().to_str_radix(10)).unwrap())
    }

    fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> String {
        assert!(
            significant_digits > 0,
            "Significant digits must be positive."
        );

        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self
            .to_decimal()
            .round_sf_with_strategy(significant_digits as u32, rounding_strategy);

        quotient.unwrap().normalize().to_string()
    }

    fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self
            .to_decimal()
            .round_dp_with_strategy(decimal_places as u32, rounding_strategy);

        format!("{:.1$}", quotient, decimal_places as usize)
    }

    /// Helper method for converting any super class back to a fraction
    fn as_fraction(&self) -> Fraction {
        Fraction::new(self.numerator().clone(), self.denominator().clone(), ())
    }
}

impl FractionTrait<()> for Fraction {
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, _: ()) -> Self {
        let denominator = denominator.into();
        assert_ne!(denominator, 0.into(), "DENOMINATOR CAN'T BE ZERO");
        Self {
            numerator: numerator.into(),
            denominator,
        }
    }

    fn meta(&self) -> () {
        ()
    }

    fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    fn denominator(&self) -> &BigInt {
        &self.denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient() {
        assert_eq!(Fraction::new(8, 3, ()).quotient(), BigInt::from(2));
        assert_eq!(Fraction::new(12, 4, ()).quotient(), BigInt::from(3));
        assert_eq!(Fraction::new(16, 5, ()).quotient(), BigInt::from(3));
    }

    #[test]
    fn test_remainder() {
        assert!(Fraction::new(8, 3, ())
            .remainder()
            .equal_to(&Fraction::new(2, 3, ())));
        assert!(Fraction::new(12, 4, ())
            .remainder()
            .equal_to(&Fraction::new(0, 4, ())));
        assert!(Fraction::new(16, 5, ())
            .remainder()
            .equal_to(&Fraction::new(1, 5, ())));
    }

    #[test]
    fn test_invert() {
        let fraction = Fraction::new(5, 10, ()).invert();
        assert_eq!(fraction.numerator, BigInt::from(10));
        assert_eq!(fraction.denominator, BigInt::from(5));
    }

    #[test]
    fn test_add() {
        assert!(Fraction::new(1, 10, ())
            .add(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(52, 120, ())));
        assert!(Fraction::new(1, 5, ())
            .add(&Fraction::new(2, 5, ()))
            .equal_to(&Fraction::new(3, 5, ())));
    }

    #[test]
    fn test_subtract() {
        assert!(Fraction::new(1, 10, ())
            .subtract(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(-28, 120, ())));
        assert!(Fraction::new(3, 5, ())
            .subtract(&Fraction::new(2, 5, ()))
            .equal_to(&Fraction::new(1, 5, ())));
    }

    #[test]
    fn test_less_than() {
        assert!(Fraction::new(1, 10, ()).less_than(&Fraction::new(4, 12, ())));
        assert!(!Fraction::new(1, 3, ()).less_than(&Fraction::new(4, 12, ())));
        assert!(!Fraction::new(5, 12, ()).less_than(&Fraction::new(4, 12, ())));
    }

    #[test]
    fn test_equal_to() {
        assert!(!Fraction::new(1, 10, ()).equal_to(&Fraction::new(4, 12, ())));
        assert!(Fraction::new(1, 3, ()).equal_to(&Fraction::new(4, 12, ())));
        assert!(!Fraction::new(5, 12, ()).equal_to(&Fraction::new(4, 12, ())));
    }

    #[test]
    fn test_greater_than() {
        assert!(!Fraction::new(1, 10, ()).greater_than(&Fraction::new(4, 12, ())));
        assert!(!Fraction::new(1, 3, ()).greater_than(&Fraction::new(4, 12, ())));
        assert!(Fraction::new(5, 12, ()).greater_than(&Fraction::new(4, 12, ())));
    }

    #[test]
    fn test_multiply() {
        assert!(Fraction::new(1, 10, ())
            .multiply(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(4, 120, ())));
        assert!(Fraction::new(1, 3, ())
            .multiply(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(4, 36, ())));
        assert!(Fraction::new(5, 12, ())
            .multiply(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(20, 144, ())));
    }

    #[test]
    fn test_divide() {
        assert!(Fraction::new(1, 10, ())
            .divide(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(12, 40, ())));
        assert!(Fraction::new(1, 3, ())
            .divide(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(12, 12, ())));
        assert!(Fraction::new(5, 12, ())
            .divide(&Fraction::new(4, 12, ()))
            .equal_to(&Fraction::new(60, 48, ())));
    }

    #[test]
    fn test_as_faction() {
        let f = Fraction::new(1, 2, ());
        // returns an equivalent but not the same reference fraction
        assert!(f.as_fraction().equal_to(&f));
        assert_ne!(&f as *const _, &f.as_fraction() as *const _);
    }
}
