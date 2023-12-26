use super::fraction::{Fraction, FractionLike, FractionTrait};
use crate::constants::Rounding;
use lazy_static::lazy_static;
use num_bigint::BigInt;

lazy_static! {
    static ref ONE_HUNDRED: Fraction = Fraction::new(100, 1);
}

/// Unit struct to distinguish between a fraction and a percent
#[derive(Clone)]
pub struct IsPercent;

pub type Percent = FractionLike<IsPercent>;

impl Percent {
    pub fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        FractionTrait::new(numerator, denominator, IsPercent)
    }

    pub fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> String {
        self.as_fraction()
            .multiply(&ONE_HUNDRED)
            .to_significant(significant_digits, rounding)
    }

    pub fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        self.as_fraction()
            .multiply(&ONE_HUNDRED)
            .to_fixed(decimal_places, rounding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::Rounding;

    #[test]
    fn test_add() {
        assert!(Percent::new(1, 100)
            .add(&Percent::new(2, 100))
            .equal_to(&Percent::new(3, 100)));
        assert!(Percent::new(1, 25)
            .add(&Percent::new(2, 100))
            .equal_to(&Percent::new(150, 2500)));
    }

    #[test]
    fn test_subtract() {
        assert!(Percent::new(1, 100)
            .subtract(&Percent::new(2, 100))
            .equal_to(&Percent::new(-1, 100)));
        assert!(Percent::new(1, 25)
            .subtract(&Percent::new(2, 100))
            .equal_to(&Percent::new(50, 2500)));
    }

    #[test]
    fn test_multiply() {
        assert!(Percent::new(1, 100)
            .multiply(&Percent::new(2, 100))
            .equal_to(&Percent::new(2, 10000)));
        assert!(Percent::new(1, 25)
            .multiply(&Percent::new(2, 100))
            .equal_to(&Percent::new(2, 2500)));
    }

    #[test]
    fn test_divide() {
        assert!(Percent::new(1, 100)
            .divide(&Percent::new(2, 100))
            .equal_to(&Percent::new(100, 200)));
        assert!(Percent::new(1, 25)
            .divide(&Percent::new(2, 100))
            .equal_to(&Percent::new(100, 50)));
    }

    #[test]
    fn test_to_significant() {
        assert_eq!(
            Percent::new(154, 10000).to_significant(3, Rounding::RoundHalfUp),
            "1.54".to_string()
        );
    }

    #[test]
    fn test_to_fixed() {
        assert_eq!(
            Percent::new(154, 10000).to_fixed(2, Rounding::RoundHalfUp),
            "1.54".to_string()
        );
    }
}
