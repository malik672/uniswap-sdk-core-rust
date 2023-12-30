// External crate dependencies
use crate::prelude::*;

// Struct representing a fraction with metadata
#[derive(Clone, PartialEq)]
pub struct FractionLike<M: Clone> {
    numerator: BigInt,
    denominator: BigInt,
    pub meta: M,
}

// Type alias for a simple Fraction without metadata
pub type Fraction = FractionLike<()>;

impl Fraction {
    // Constructor for creating a new Fraction instance
    pub fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        FractionTrait::new(numerator, denominator, ())
    }
}

// Function to convert the custom Rounding enum to `bigdecimal`'s `RoundingMode`
fn to_rounding_strategy(rounding: Rounding) -> RoundingMode {
    match rounding {
        Rounding::RoundDown => RoundingMode::Down,
        Rounding::RoundHalfUp => RoundingMode::HalfUp,
        Rounding::RoundUp => RoundingMode::Up,
    }
}

// Trait defining common operations for fractions with metadata
pub trait FractionTrait<M>: Sized {
    // Constructor method for creating a new Fraction with metadata
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self;

    // Accessor method for retrieving metadata
    fn meta(&self) -> M;

    // Accessor method for retrieving the numerator
    fn numerator(&self) -> &BigInt;

    // Accessor method for retrieving the denominator
    fn denominator(&self) -> &BigInt;

    // Returns the floor division quotient of the fraction
    fn quotient(&self) -> BigInt {
        self.numerator().div_floor(self.denominator())
    }

    // Returns the remainder after floor division as a new fraction
    fn remainder(&self) -> Self {
        Self::new(
            self.numerator() % self.denominator(),
            self.denominator().clone(),
            self.meta(),
        )
    }

    // Returns the inverted fraction
    fn invert(&self) -> Self {
        Self::new(
            self.denominator().clone(),
            self.numerator().clone(),
            self.meta(),
        )
    }

    // Adds another fraction to the current fraction
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

    // Subtracts another fraction from the current fraction
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

    // Multiplies the current fraction by another fraction
    fn multiply(&self, other: &Self) -> Self {
        Self::new(
            self.numerator() * other.numerator(),
            self.denominator() * other.denominator(),
            self.meta(),
        )
    }

    // Divides the current fraction by another fraction
    fn divide(&self, other: &Self) -> Self {
        Self::new(
            self.numerator() * other.denominator(),
            self.denominator() * other.numerator(),
            self.meta(),
        )
    }

    // Checks if the current fraction is less than another fraction
    fn less_than(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() < other.numerator() * self.denominator()
    }

    // Checks if the current fraction is equal to another fraction
    fn equal_to(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() == other.numerator() * self.denominator()
    }

    // Checks if the current fraction is greater than another fraction
    fn greater_than(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() > other.numerator() * self.denominator()
    }

    // Converts the fraction to a `bigdecimal::BigDecimal`
    fn to_decimal(&self) -> BigDecimal {
        BigDecimal::from_str(&self.numerator().to_str_radix(10))
            .unwrap()
            .div(BigDecimal::from_str(&self.denominator().to_str_radix(10)).unwrap())
    }

    // Converts the fraction to a string with a specified number of significant digits and rounding strategy
    fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> String {
        assert!(
            significant_digits > 0,
            "Significant digits must be positive."
        );

        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self.to_decimal().with_precision_round(
            NonZeroU64::new(significant_digits as u64).unwrap(),
            rounding_strategy,
        );

        quotient.normalized().to_string()
    }

    // Converts the fraction to a string with a fixed number of decimal places and rounding strategy
    fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self
            .to_decimal()
            .with_scale_round(decimal_places as i64, rounding_strategy);

        format!("{:.1$}", quotient, decimal_places as usize)
    }

    // Helper method for converting any superclass back to a simple Fraction
    fn as_fraction(&self) -> Fraction {
        Fraction::new(self.numerator().clone(), self.denominator().clone())
    }
}

// Implementation of the FractionTrait for FractionLike
impl<M: Clone> FractionTrait<M> for FractionLike<M> {
    // Constructor for creating a new Fraction with metadata
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self {
        let denominator = denominator.into();
        // Ensure the denominator is not zero
        assert!(!denominator.is_zero(), "DENOMINATOR CAN'T BE ZERO");
        Self {
            numerator: numerator.into(),
            denominator,
            meta,
        }
    }

    // Accessor method for retrieving metadata
    fn meta(&self) -> M {
        self.meta.clone()
    }

    // Accessor method for retrieving the numerator
    fn numerator(&self) -> &BigInt {
        &self.numerator
    }

    // Accessor method for retrieving the denominator
    fn denominator(&self) -> &BigInt {
        &self.denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient() {
        assert_eq!(Fraction::new(8, 3).quotient(), BigInt::from(2));
        assert_eq!(Fraction::new(12, 4).quotient(), BigInt::from(3));
        assert_eq!(Fraction::new(16, 5).quotient(), BigInt::from(3));
    }

    #[test]
    fn test_remainder() {
        assert!(Fraction::new(8, 3)
            .remainder()
            .equal_to(&Fraction::new(2, 3)));
        assert!(Fraction::new(12, 4)
            .remainder()
            .equal_to(&Fraction::new(0, 4)));
        assert!(Fraction::new(16, 5)
            .remainder()
            .equal_to(&Fraction::new(1, 5)));
    }

    #[test]
    fn test_invert() {
        let fraction = Fraction::new(5, 10).invert();
        assert_eq!(fraction.numerator, BigInt::from(10));
        assert_eq!(fraction.denominator, BigInt::from(5));
    }

    #[test]
    fn test_add() {
        assert!(Fraction::new(1, 10)
            .add(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(52, 120)));
        assert!(Fraction::new(1, 5)
            .add(&Fraction::new(2, 5))
            .equal_to(&Fraction::new(3, 5)));
    }

    #[test]
    fn test_subtract() {
        assert!(Fraction::new(1, 10)
            .subtract(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(-28, 120)));
        assert!(Fraction::new(3, 5)
            .subtract(&Fraction::new(2, 5))
            .equal_to(&Fraction::new(1, 5)));
    }

    #[test]
    fn test_less_than() {
        assert!(Fraction::new(1, 10).less_than(&Fraction::new(4, 12)));
        assert!(!Fraction::new(1, 3).less_than(&Fraction::new(4, 12)));
        assert!(!Fraction::new(5, 12).less_than(&Fraction::new(4, 12)));
    }

    #[test]
    fn test_equal_to() {
        assert!(!Fraction::new(1, 10).equal_to(&Fraction::new(4, 12)));
        assert!(Fraction::new(1, 3).equal_to(&Fraction::new(4, 12)));
        assert!(!Fraction::new(5, 12).equal_to(&Fraction::new(4, 12)));
    }

    #[test]
    fn test_greater_than() {
        assert!(!Fraction::new(1, 10).greater_than(&Fraction::new(4, 12)));
        assert!(!Fraction::new(1, 3).greater_than(&Fraction::new(4, 12)));
        assert!(Fraction::new(5, 12).greater_than(&Fraction::new(4, 12)));
    }

    #[test]
    fn test_multiply() {
        assert!(Fraction::new(1, 10)
            .multiply(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(4, 120)));
        assert!(Fraction::new(1, 3)
            .multiply(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(4, 36)));
        assert!(Fraction::new(5, 12)
            .multiply(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(20, 144)));
    }

    #[test]
    fn test_divide() {
        assert!(Fraction::new(1, 10)
            .divide(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(12, 40)));
        assert!(Fraction::new(1, 3)
            .divide(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(12, 12)));
        assert!(Fraction::new(5, 12)
            .divide(&Fraction::new(4, 12))
            .equal_to(&Fraction::new(60, 48)));
    }

    #[test]
    fn test_as_faction() {
        let f = Fraction::new(1, 2);
        // returns an equivalent but not the same reference fraction
        assert!(f.as_fraction().equal_to(&f));
        assert_ne!(&f as *const _, &f.as_fraction() as *const _);
    }
}
