/// External crate dependencies
use crate::prelude::*;
use std::ops::{Add, Deref, Div, Mul, Sub};

/// Struct representing a fraction with metadata
#[derive(Clone, Debug)]
pub struct FractionLike<M> {
    numerator: BigInt,
    denominator: BigInt,
    pub meta: M,
}

impl<M: Default> Default for FractionLike<M> {
    fn default() -> Self {
        Self {
            numerator: BigInt::from(0),
            denominator: BigInt::from(1),
            meta: M::default(),
        }
    }
}

impl<M> Deref for FractionLike<M> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}

/// Type alias for a simple Fraction without metadata
pub type Fraction = FractionLike<()>;

impl Fraction {
    // Constructor for creating a new Fraction instance
    pub fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        FractionBase::new(numerator, denominator, ())
    }
}

/// Function to convert the custom Rounding enum to `bigdecimal`'s `RoundingMode`
const fn to_rounding_strategy(rounding: Rounding) -> RoundingMode {
    match rounding {
        Rounding::RoundDown => RoundingMode::Down,
        Rounding::RoundHalfUp => RoundingMode::HalfUp,
        Rounding::RoundUp => RoundingMode::Up,
    }
}

/// Trait defining common operations for fractions with metadata
pub trait FractionTrait<M>
where
    Self: FractionBase<M>
        + Ord
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>,
{
}

/// Trait defining common operations for fractions with metadata
pub trait FractionBase<M>: Sized {
    /// Constructor method for creating a new Fraction with metadata
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self;

    /// Accessor method for retrieving metadata
    fn meta(&self) -> M;

    /// Accessor method for retrieving the numerator
    fn numerator(&self) -> BigInt;

    /// Accessor method for retrieving the denominator
    fn denominator(&self) -> BigInt;

    /// Returns the floor division quotient of the fraction
    fn quotient(&self) -> BigInt {
        self.numerator().div_floor(&self.denominator())
    }

    /// Returns the remainder after floor division as a new fraction
    fn remainder(&self) -> Self {
        Self::new(
            self.numerator() % self.denominator(),
            self.denominator(),
            self.meta(),
        )
    }

    /// Returns the inverted fraction
    fn invert(&self) -> Self {
        Self::new(self.denominator(), self.numerator(), self.meta())
    }

    /// Converts the fraction to a `bigdecimal::BigDecimal`
    fn to_decimal(&self) -> BigDecimal {
        BigDecimal::from(self.numerator()).div(BigDecimal::from(self.denominator()))
    }

    /// Converts the fraction to a string with a specified number of significant digits and rounding strategy
    fn to_significant(&self, significant_digits: u8, rounding: Rounding) -> Result<String, Error> {
        if significant_digits == 0 {
            return Err(Error::Incorrect());
        }
        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self.to_decimal().with_precision_round(
            NonZeroU64::new(significant_digits as u64).unwrap(),
            rounding_strategy,
        );

        Ok(quotient.normalized().to_string())
    }

    /// Converts the fraction to a string with a fixed number of decimal places and rounding strategy
    fn to_fixed(&self, decimal_places: u8, rounding: Rounding) -> String {
        let rounding_strategy = to_rounding_strategy(rounding);
        let quotient = self
            .to_decimal()
            .with_scale_round(decimal_places as i64, rounding_strategy);

        format!("{:.1$}", quotient, decimal_places as usize)
    }

    /// Helper method for converting any superclass back to a simple Fraction
    fn as_fraction(&self) -> Fraction {
        Fraction::new(self.numerator(), self.denominator())
    }
}

/// Implementation of the FractionTrait for FractionLike
impl<M: Clone + PartialEq> FractionTrait<M> for FractionLike<M> {}

impl<M: Clone> FractionBase<M> for FractionLike<M> {
    /// Constructor for creating a new Fraction with metadata
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self {
        let denominator = denominator.into();
        // Ensure the denominator is not zero
        if denominator.is_zero() {
            panic!("denominator is zero");
        }
        Self {
            numerator: numerator.into(),
            denominator,
            meta,
        }
    }

    /// Accessor method for retrieving metadata
    fn meta(&self) -> M {
        self.meta.clone()
    }

    /// Accessor method for retrieving the numerator
    fn numerator(&self) -> BigInt {
        self.numerator.clone()
    }

    /// Accessor method for retrieving the denominator
    fn denominator(&self) -> BigInt {
        self.denominator.clone()
    }
}

impl<M> PartialEq for FractionLike<M>
where
    M: Clone + PartialEq,
{
    /// Checks if the current fraction is equal to another fraction
    fn eq(&self, other: &Self) -> bool {
        self.numerator() * other.denominator() == other.numerator() * self.denominator()
            && self.meta() == other.meta()
    }
}

impl<M: Clone + PartialEq> Eq for FractionLike<M> {}

impl<M> Ord for FractionLike<M>
where
    M: Clone + PartialEq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        (self.numerator() * other.denominator()).cmp(&(other.numerator() * self.denominator()))
    }
}

impl<M> PartialOrd<Self> for FractionLike<M>
where
    M: Clone + PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<M: Clone> Add for FractionLike<M> {
    type Output = Self;

    /// Adds another fraction to the current fraction
    fn add(self, other: Self) -> Self::Output {
        if self.denominator == other.denominator() {
            FractionBase::new(
                self.numerator() + other.numerator(),
                self.denominator(),
                self.meta(),
            )
        } else {
            FractionBase::new(
                self.numerator() * other.denominator() + other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }
}

impl<M: Clone> Sub for FractionLike<M> {
    type Output = Self;

    /// Subtracts another fraction from the current fraction
    fn sub(self, other: Self) -> Self::Output {
        if self.denominator == other.denominator() {
            FractionBase::new(
                self.numerator() - other.numerator(),
                self.denominator(),
                self.meta(),
            )
        } else {
            FractionBase::new(
                self.numerator() * other.denominator() - other.numerator() * self.denominator(),
                self.denominator() * other.denominator(),
                self.meta(),
            )
        }
    }
}

impl<M: Clone> Mul for FractionLike<M> {
    type Output = Self;

    /// Multiplies the current fraction by another fraction
    fn mul(self, other: Self) -> Self::Output {
        //There's little to no possibility of an error, so unwrap can be used
        FractionBase::new(
            self.numerator() * other.numerator(),
            self.denominator() * other.denominator(),
            self.meta(),
        )
    }
}

impl<M: Clone> Div for FractionLike<M> {
    type Output = Self;

    /// Divides the current fraction by another fraction
    /// There's little to no possibility of an error, so unwrap can be used
    fn div(self, other: Self) -> Self::Output {
        FractionBase::new(
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
        assert_eq!(Fraction::new(8, 3).quotient(), BigInt::from(2));
        assert_eq!(Fraction::new(12, 4).quotient(), BigInt::from(3));
        assert_eq!(Fraction::new(16, 5).quotient(), BigInt::from(3));
    }

    #[test]
    fn test_remainder() {
        assert_eq!(Fraction::new(8, 3).remainder(), Fraction::new(2, 3));
        assert_eq!(Fraction::new(12, 4).remainder(), Fraction::default());
        assert_eq!(Fraction::new(16, 5).remainder(), Fraction::new(1, 5));
    }

    #[test]
    fn test_invert() {
        let fraction = Fraction::new(5, 10).invert();
        assert_eq!(fraction.numerator, BigInt::from(10));
        assert_eq!(fraction.denominator, BigInt::from(5));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Fraction::new(1, 10) + Fraction::new(4, 12),
            Fraction::new(52, 120)
        );

        assert_eq!(
            Fraction::new(1, 5) + Fraction::new(2, 5),
            Fraction::new(3, 5)
        );
    }

    #[test]
    fn test_subtract() {
        assert_eq!(
            Fraction::new(1, 10) - Fraction::new(4, 12),
            Fraction::new(-28, 120)
        );
        assert_eq!(
            Fraction::new(3, 5) - Fraction::new(2, 5),
            Fraction::new(1, 5)
        );
    }

    #[test]
    fn test_less_than() {
        assert!(Fraction::new(1, 10) < Fraction::new(4, 12));
        assert!(Fraction::new(1, 3) >= Fraction::new(4, 12));
        assert!(Fraction::new(5, 12) >= Fraction::new(4, 12));
    }

    #[test]
    fn test_equal_to() {
        assert_ne!(Fraction::new(1, 10), Fraction::new(4, 12));
        assert_eq!(Fraction::new(1, 3), Fraction::new(4, 12));
        assert_ne!(Fraction::new(5, 12), Fraction::new(4, 12));
    }

    #[test]
    fn test_greater_than() {
        assert!(Fraction::new(1, 10) <= Fraction::new(4, 12));
        assert!(Fraction::new(1, 3) <= Fraction::new(4, 12));
        assert!(Fraction::new(5, 12) > Fraction::new(4, 12));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(
            (Fraction::new(1, 10) * Fraction::new(4, 12)),
            Fraction::new(4, 120)
        );
        assert_eq!(
            (Fraction::new(1, 3) * Fraction::new(4, 12)),
            Fraction::new(4, 36)
        );
        assert_eq!(
            (Fraction::new(5, 12) * Fraction::new(4, 12)),
            Fraction::new(20, 144)
        );
    }

    #[test]
    fn test_divide() {
        assert_eq!(
            Fraction::new(1, 10) / Fraction::new(4, 12),
            Fraction::new(12, 40)
        );
        assert_eq!(
            Fraction::new(1, 3) / Fraction::new(4, 12),
            Fraction::new(12, 12)
        );
        assert_eq!(
            Fraction::new(5, 12) / Fraction::new(4, 12),
            Fraction::new(60, 48)
        );
    }

    #[test]
    fn test_as_faction() {
        let f = Fraction::new(1, 2);
        // returns an equivalent but not the same reference fraction
        assert_eq!(f.as_fraction(), f);
        assert_ne!(&f as *const _, &f.as_fraction() as *const _);
    }
}
