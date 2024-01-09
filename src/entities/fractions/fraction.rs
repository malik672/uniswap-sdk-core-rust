// External crate dependencies
use crate::prelude::*;
use std::ops::{Add, Div, Mul, Sub};

// Struct representing a fraction with metadata
#[derive(Clone, Debug)]
pub struct FractionLike<M: Clone> {
    numerator: BigInt,
    denominator: BigInt,
    pub meta: M,
}

// Type alias for a simple Fraction without metadata
pub type Fraction = FractionLike<()>;

impl Fraction {
    // Constructor for creating a new Fraction instance
    pub fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
    ) -> Result<Self, Error> {
        FractionTrait::new(numerator, denominator, ())
    }
}

// Function to convert the custom Rounding enum to `bigdecimal`'s `RoundingMode`
const fn to_rounding_strategy(rounding: Rounding) -> RoundingMode {
    match rounding {
        Rounding::RoundDown => RoundingMode::Down,
        Rounding::RoundHalfUp => RoundingMode::HalfUp,
        Rounding::RoundUp => RoundingMode::Up,
    }
}

// Trait defining common operations for fractions with metadata
pub trait FractionTrait<M>
where
    Self: Sized
        + Ord
        + Add<Output = Result<Self, Error>>
        + Sub<Output = Result<Self, Error>>
        + Mul<Output = Result<Self, Error>>
        + Div<Output = Result<Self, Error>>,
{
    // Constructor method for creating a new Fraction with metadata
    fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
        meta: M,
    ) -> Result<Self, Error>;

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
    fn remainder(&self) -> Result<Self, Error> {
        Self::new(
            self.numerator() % self.denominator(),
            self.denominator().clone(),
            self.meta(),
        )
        .map_err(|err| Error::CreationError(format!("{}", err)))
    }

    // Returns the inverted fraction
    fn invert(&self) -> Result<Self, Error> {
        Self::new(
            self.denominator().clone(),
            self.numerator().clone(),
            self.meta(),
        )
        .map_err(|err| Error::CreationError(format!("{}", err)))
    }

    // Converts the fraction to a `bigdecimal::BigDecimal`
    fn to_decimal(&self) -> BigDecimal {
        BigDecimal::from_str(&self.numerator().to_str_radix(10))
            .unwrap()
            .div(BigDecimal::from_str(&self.denominator().to_str_radix(10)).unwrap())
    }

    // Converts the fraction to a string with a specified number of significant digits and rounding strategy
    fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> Result<String, Error> {
        if significant_digits == 0 {
            return Err(Error::Incorrect(format!(
                "significant digits:{} should always be positive",
                significant_digits
            )));
        }
        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self.to_decimal().with_precision_round(
            NonZeroU64::new(significant_digits as u64).unwrap(),
            rounding_strategy,
        );

        Ok(quotient.normalized().to_string())
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
    fn as_fraction(&self) -> Result<Fraction, Error> {
        Fraction::new(self.numerator().clone(), self.denominator().clone())
            .map_err(|err| Error::CreateFractionalError(format!("{}", err)))
    }
}

// Implementation of the FractionTrait for FractionLike
impl<M: Clone> FractionTrait<M> for FractionLike<M> {
    // Constructor for creating a new Fraction with metadata
    fn new(
        numerator: impl Into<BigInt>,
        denominator: impl Into<BigInt>,
        meta: M,
    ) -> Result<Self, Error> {
        let denominator = denominator.into();
        // Ensure the denominator is not zero
        if denominator.is_zero() {
            return Err(Error::Incorrect("denominator can't be zero".to_owned()));
        }
        Ok(Self {
            numerator: numerator.into(),
            denominator,
            meta,
        })
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

impl<M: Clone> PartialEq for FractionLike<M> {
    // Checks if the current fraction is equal to another fraction
    fn eq(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() == other.numerator() * self.denominator()
    }
}

impl<M: Clone> Eq for FractionLike<M> {}

impl<M: Clone> Ord for FractionLike<M> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator() * other.denominator()).cmp(&(other.numerator() * self.denominator()))
    }
}

impl<M: Clone> PartialOrd<Self> for FractionLike<M> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: Clone> Add for FractionLike<M> {
    type Output = Result<Self, Error>;

    // Adds another fraction to the current fraction
    fn add(self, other: Self) -> Self::Output {
        if self.denominator() == other.denominator() {
            FractionTrait::new(
                self.numerator() + other.numerator(),
                self.denominator().clone(),
                self.meta(),
            )
        } else {
            FractionTrait::new(
                self.numerator() * other.denominator() + other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }
}

impl<M: Clone> Sub for FractionLike<M> {
    type Output = Result<Self, Error>;

    // Subtracts another fraction from the current fraction
    fn sub(self, other: Self) -> Self::Output {
        if self.denominator() == other.denominator() {
            FractionTrait::new(
                self.numerator() - other.numerator(),
                self.denominator().clone(),
                self.meta(),
            )
        } else {
            FractionTrait::new(
                self.numerator() * other.denominator() - other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }
}

impl<M: Clone> Mul for FractionLike<M> {
    type Output = Result<Self, Error>;

    // Multiplies the current fraction by another fraction
    fn mul(self, other: Self) -> Self::Output {
        FractionTrait::new(
            self.numerator() * other.numerator(),
            self.denominator() * other.denominator(),
            self.meta(),
        )
    }
}

impl<M: Clone> Div for FractionLike<M> {
    type Output = Result<Self, Error>;

    // Divides the current fraction by another fraction
    fn div(self, other: Self) -> Self::Output {
        FractionTrait::new(
            self.numerator() * other.denominator(),
            self.denominator() * other.numerator(),
            self.meta(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quotient() {
        assert_eq!(Fraction::new(8, 3).unwrap().quotient(), BigInt::from(2));
        assert_eq!(Fraction::new(12, 4).unwrap().quotient(), BigInt::from(3));
        assert_eq!(Fraction::new(16, 5).unwrap().quotient(), BigInt::from(3));
    }

    #[test]
    fn test_remainder() {
        assert_eq!(
            Fraction::new(8, 3).unwrap().remainder().unwrap(),
            Fraction::new(2, 3).unwrap()
        );
        assert_eq!(
            Fraction::new(12, 4).unwrap().remainder().unwrap(),
            Fraction::new(0, 4).unwrap()
        );
        assert_eq!(
            Fraction::new(16, 5).unwrap().remainder().unwrap(),
            Fraction::new(1, 5).unwrap()
        );
    }

    #[test]
    fn test_invert() {
        let fraction = Fraction::new(5, 10).unwrap().invert().unwrap();
        assert_eq!(fraction.clone().numerator, BigInt::from(10));
        assert_eq!(fraction.clone().denominator, BigInt::from(5));
    }

    #[test]
    fn test_add() {
        let fraction1 = Fraction::new(1, 10).unwrap();
        let fraction2 = Fraction::new(4, 12).unwrap();
        let expected = Fraction::new(52, 120).unwrap();
        let add = fraction1 + fraction2;
        assert_eq!(add.unwrap(), expected);

        let fraction3 = Fraction::new(1, 5).unwrap();
        let fraction4 = Fraction::new(2, 5).unwrap();
        let expected2: FractionLike<()> = Fraction::new(3, 5).unwrap();
        let add2 = fraction3 + fraction4;
        assert_eq!(add2.unwrap(), expected2);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            (Fraction::new(1, 10).unwrap() - Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(-28, 120).unwrap()
        );
        assert_eq!(
            (Fraction::new(3, 5).unwrap() - Fraction::new(2, 5).unwrap()).unwrap(),
            Fraction::new(1, 5).unwrap()
        );
    }

    #[test]
    fn test_less_than() {
        assert!(Fraction::new(1, 10).unwrap() < Fraction::new(4, 12).unwrap());
        assert!(!(Fraction::new(1, 3).unwrap() < Fraction::new(4, 12).unwrap()));
        assert!(!(Fraction::new(5, 12).unwrap() < Fraction::new(4, 12).unwrap()));
    }

    #[test]
    fn test_equal_to() {
        assert_ne!(Fraction::new(1, 10).unwrap(), Fraction::new(4, 12).unwrap());
        assert_eq!(Fraction::new(1, 3).unwrap(), Fraction::new(4, 12).unwrap());
        assert_ne!(Fraction::new(5, 12).unwrap(), Fraction::new(4, 12).unwrap());
    }

    #[test]
    fn test_greater_than() {
        assert!(!(Fraction::new(1, 10).unwrap() > Fraction::new(4, 12).unwrap()));
        assert!(!(Fraction::new(1, 3).unwrap() > Fraction::new(4, 12).unwrap()));
        assert!(Fraction::new(5, 12).unwrap() > Fraction::new(4, 12).unwrap());
    }

    #[test]
    fn test_multiply() {
        assert_eq!(
            (Fraction::new(1, 10).unwrap() * Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(4, 120).unwrap()
        );
        assert_eq!(
            (Fraction::new(1, 3).unwrap() * Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(4, 36).unwrap()
        );
        assert_eq!(
            (Fraction::new(5, 12).unwrap() * Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(20, 144).unwrap()
        );
    }

    #[test]
    fn test_divide() {
        assert_eq!(
            (Fraction::new(1, 10).unwrap() / Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(12, 40).unwrap()
        );
        assert_eq!(
            (Fraction::new(1, 3).unwrap() / Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(12, 12).unwrap()
        );
        assert_eq!(
            (Fraction::new(5, 12).unwrap() / Fraction::new(4, 12).unwrap()).unwrap(),
            Fraction::new(60, 48).unwrap()
        );
    }

    #[test]
    fn test_as_faction() {
        let f = Fraction::new(1, 2).unwrap();
        // returns an equivalent but not the same reference fraction
        assert_eq!(f.clone().as_fraction().unwrap(), f.clone());
        assert_ne!(
            &f.clone() as *const _,
            &f.clone().as_fraction().unwrap() as *const _
        );
    }
}
