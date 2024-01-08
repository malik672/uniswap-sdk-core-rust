// Importing dependencies from the same module
use crate::prelude::*;

// Lazily initialized constant representing the fraction 100/1
lazy_static! {
    static ref ONE_HUNDRED: Fraction = Fraction::new(100, 1).unwrap();
}

/// Unit struct to distinguish between a fraction and a percent
#[derive(Clone, Debug)]
pub struct IsPercent;

// Type alias for a Percent, a Fraction with the IsPercent metadata
pub type Percent = FractionLike<IsPercent>;

impl Percent {
    /// Constructor for creating a new Percent instance
    pub fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        FractionTrait::new(numerator, denominator, IsPercent)
            .map_err(|err| Error::CreationError(format!("{}", err)))
    }

    /// Converts the Percent to a string with a specified number of significant digits and rounding strategy
    pub fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> String {
        // Convert the Percent to a simple Fraction, multiply by 100, and then call to_significant on the result
        //Unwrap is used here, because there's little to  no possibility of failure
        (self.as_fraction().unwrap() * ONE_HUNDRED.clone())
            .unwrap()
            .to_significant(significant_digits, rounding)
            .unwrap()
    }

    /// Converts the Percent to a string with a fixed number of decimal places and rounding strategy
    pub fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        // Convert the Percent to a simple Fraction, multiply by 100, and then call to_fixed on the
        //Unwrap is used here, because there's little to  no possibility of failure
        (self.as_fraction().unwrap() * ONE_HUNDRED.clone())
            .unwrap()
            .to_fixed(decimal_places, rounding)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            (Percent::new(1, 100).unwrap() + Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(3, 100).unwrap()
        );
        assert_eq!(
            (Percent::new(1, 25).unwrap() + Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(150, 2500).unwrap()
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            (Percent::new(1, 100).unwrap() - Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(-1, 100).unwrap()
        );
        assert_eq!(
            (Percent::new(1, 25).unwrap() - Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(50, 2500).unwrap()
        );
    }

    #[test]
    fn test_multiply() {
        assert_eq!(
            (Percent::new(1, 100).unwrap() * Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(2, 10000).unwrap()
        );
        assert_eq!(
            (Percent::new(1, 25).unwrap() * Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(2, 2500).unwrap()
        );
    }

    #[test]
    fn test_divide() {
        assert_eq!(
            (Percent::new(1, 100).unwrap() / Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(100, 200).unwrap()
        );
        assert_eq!(
            (Percent::new(1, 25).unwrap() / Percent::new(2, 100).unwrap()).unwrap(),
            Percent::new(100, 50).unwrap()
        );
    }

    #[test]
    fn test_to_significant() {
        assert_eq!(
            Percent::new(154, 10000)
                .unwrap()
                .to_significant(3, Rounding::RoundHalfUp),
            "1.54".to_string()
        );
    }

    #[test]
    fn test_to_fixed() {
        assert_eq!(
            Percent::new(154, 10000)
                .unwrap()
                .to_fixed(2, Rounding::RoundHalfUp),
            "1.54".to_string()
        );
    }
}
