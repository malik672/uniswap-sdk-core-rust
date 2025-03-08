use crate::prelude::*;
use alloc::string::ToString;
use bnum::cast::CastFrom;
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Add, Div, Mul, Sub},
};
use derive_more::Deref;
use fastnum::{i512, I1024};
use num_integer::Integer;

/// Struct representing a fraction with metadata
#[derive(Clone, Debug, Deref)]
pub struct FractionLike<M> {
    pub numerator: BigInt,
    pub denominator: BigInt,
    /// Metadata associated with the fraction
    #[deref]
    pub meta: M,
}

impl<M: Default> Default for FractionLike<M> {
    #[inline]
    fn default() -> Self {
        Self {
            numerator: BigInt::ZERO,
            denominator: i512!(1),
            meta: M::default(),
        }
    }
}

/// Type alias for a simple Fraction without metadata
pub type Fraction = FractionLike<()>;

impl Fraction {
    /// Creates a new `Fraction` instance with the given numerator and denominator.
    ///
    /// # Arguments
    ///
    /// * `numerator` - The numerator of the fraction.
    /// * `denominator` - The denominator of the fraction.
    ///
    /// # Returns
    ///
    /// A new `Fraction` instance with the specified numerator and denominator.
    ///
    /// # Panics
    ///
    /// This function will panic if the denominator is zero.
    #[inline]
    pub fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>) -> Self {
        FractionBase::new(numerator, denominator, ())
    }
}

/// Function to convert the custom Rounding enum to [`RoundingMode`]
#[inline]
const fn to_rounding_strategy(rounding: Rounding) -> RoundingMode {
    match rounding {
        Rounding::RoundDown => RoundingMode::Down,
        Rounding::RoundHalfUp => RoundingMode::HalfUp,
        Rounding::RoundUp => RoundingMode::Up,
    }
}

/// Trait defining common operations for fractions with metadata
pub trait FractionTrait<M: Clone>
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
pub trait FractionBase<M: Clone>: Sized {
    /// Constructor method for creating a new Fraction with metadata
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self;

    /// Accessor method for retrieving metadata
    fn meta(&self) -> &M;

    /// Accessor method for retrieving numerator
    fn numerator(&self) -> BigInt;

    /// Accessor method for retrieving the denominator
    fn denominator(&self) -> BigInt;

    /// Returns the floor division quotient of the fraction
    #[inline]
    fn quotient(&self) -> BigInt {
        self.numerator().div_floor(self.denominator())
    }

    /// Returns the remainder after floor division as a new fraction
    #[inline]
    fn remainder(&self) -> Self {
        Self::new(
            self.numerator() % self.denominator(),
            self.denominator(),
            self.meta().clone(),
        )
    }

    /// Returns the inverted fraction
    #[inline]
    fn invert(&self) -> Self {
        Self::new(self.denominator(), self.numerator(), self.meta().clone())
    }

    /// Converts the fraction to a [`BigDecimal`]
    #[inline]
    fn to_decimal(&self) -> BigDecimal {
        self.numerator().to_big_decimal() / self.denominator().to_big_decimal()
    }

    /// Converts the fraction to a string with a specified number of significant digits and rounding
    /// strategy
    #[inline]
    fn to_significant(
        &self,
        significant_digits: u8,
        rounding: Option<Rounding>,
    ) -> Result<String, Error> {
        if significant_digits == 0 {
            return Err(Error::Invalid("SIGNIFICANT_DIGITS"));
        }
        let rounding_strategy = to_rounding_strategy(rounding.unwrap_or_default());
        let quotient = self.to_decimal().with_rounding_mode(rounding_strategy);
        let integer_digits = quotient.digits_count() as i16 - quotient.fractional_digits_count();
        let quotient = quotient
            .round(significant_digits as i16 - integer_digits)
            .reduce();

        Ok(quotient.to_string())
    }

    /// Converts the fraction to a string with a fixed number of decimal places and rounding
    /// strategy
    #[inline]
    fn to_fixed(&self, decimal_places: u8, rounding: Option<Rounding>) -> String {
        let rounding_strategy = to_rounding_strategy(rounding.unwrap_or_default());
        self.to_decimal()
            .with_rounding_mode(rounding_strategy)
            .round(decimal_places as i16)
            .to_string()
    }

    /// Helper method for converting any superclass back to a simple [`Fraction`]
    #[inline]
    fn as_fraction(&self) -> Fraction {
        Fraction::new(self.numerator(), self.denominator())
    }
}

impl<M: Clone + PartialEq> FractionTrait<M> for FractionLike<M> {}

impl<M: Clone> FractionBase<M> for FractionLike<M> {
    /// Constructor for creating a new [`FractionLike`] with metadata
    #[inline]
    fn new(numerator: impl Into<BigInt>, denominator: impl Into<BigInt>, meta: M) -> Self {
        let denominator = denominator.into();
        assert_ne!(denominator, BigInt::ZERO, "denominator is zero");
        Self {
            numerator: numerator.into(),
            denominator,
            meta,
        }
    }

    /// Accessor method for retrieving metadata
    #[inline]
    fn meta(&self) -> &M {
        &self.meta
    }

    /// Accessor method for retrieving the numerator
    #[inline]
    fn numerator(&self) -> BigInt {
        self.numerator
    }

    /// Accessor method for retrieving the denominator
    #[inline]
    fn denominator(&self) -> BigInt {
        self.denominator
    }
}

impl<M: PartialEq> PartialEq for FractionLike<M> {
    /// Checks if the current fraction is equal to another fraction
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        I1024::cast_from(self.numerator) * I1024::cast_from(other.denominator)
            == I1024::cast_from(other.numerator) * I1024::cast_from(self.denominator)
            && self.meta == other.meta
    }
}

impl<M: PartialEq> Eq for FractionLike<M> {}

impl<M: Hash> Hash for FractionLike<M> {
    /// Hashes the fraction using the numerator, denominator, and metadata
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.numerator.hash(state);
        self.denominator.hash(state);
        self.meta.hash(state);
    }
}

impl<M: PartialEq> Ord for FractionLike<M> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (I1024::cast_from(self.numerator) * I1024::cast_from(other.denominator))
            .cmp(&(I1024::cast_from(other.numerator) * I1024::cast_from(self.denominator)))
    }
}

impl<M: PartialEq> PartialOrd<Self> for FractionLike<M> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_add_sub {
    ($trait:ident, $method:ident, $op:tt, $Rhs:ty) => {
        impl<M: Clone> $trait<$Rhs> for FractionLike<M> {
            type Output = Self;

            #[inline]
            fn $method(self, other: $Rhs) -> Self::Output {
                if self.denominator == other.denominator {
                    let numerator = self.numerator $op other.numerator;
                    let gcd = numerator.gcd(&self.denominator);
                    FractionBase::new(
                        numerator / gcd,
                        self.denominator / gcd,
                        self.meta,
                    )
                } else {
                    let numerator = I1024::cast_from(self.numerator) * I1024::cast_from(other.denominator) $op I1024::cast_from(other.numerator) * I1024::cast_from(self.denominator);
                    let denominator = I1024::cast_from(self.denominator) * I1024::cast_from(other.denominator);
                    let gcd = numerator.gcd(&denominator);
                    FractionBase::new(
                        BigInt::cast_from(numerator / gcd),
                        BigInt::cast_from(denominator / gcd),
                        self.meta,
                    )
                }
            }
        }
    };
}

impl_add_sub!(Add, add, +, Self);
impl_add_sub!(Add, add, +, &Self);
impl_add_sub!(Sub, sub, -, Self);
impl_add_sub!(Sub, sub, -, &Self);

macro_rules! impl_mul {
    ($trait:ident, $method:ident, $Rhs:ty) => {
        impl<M: Clone> $trait<$Rhs> for FractionLike<M> {
            type Output = Self;

            #[inline]
            fn $method(self, other: $Rhs) -> Self::Output {
                let numerator =
                    I1024::cast_from(self.numerator) * I1024::cast_from(other.numerator);
                let denominator =
                    I1024::cast_from(self.denominator) * I1024::cast_from(other.denominator);
                let gcd = numerator.gcd(&denominator);
                FractionBase::new(
                    BigInt::cast_from(numerator / gcd),
                    BigInt::cast_from(denominator / gcd),
                    self.meta,
                )
            }
        }
    };
}

impl_mul!(Mul, mul, Self);
impl_mul!(Mul, mul, &Self);

macro_rules! impl_div {
    ($trait:ident, $method:ident, $Rhs:ty) => {
        impl<M: Clone> $trait<$Rhs> for FractionLike<M> {
            type Output = Self;

            #[inline]
            fn $method(self, other: $Rhs) -> Self::Output {
                let numerator =
                    I1024::cast_from(self.numerator) * I1024::cast_from(other.denominator);
                let denominator =
                    I1024::cast_from(self.denominator) * I1024::cast_from(other.numerator);
                let gcd = numerator.gcd(&denominator);
                FractionBase::new(
                    BigInt::cast_from(numerator / gcd),
                    BigInt::cast_from(denominator / gcd),
                    self.meta,
                )
            }
        }
    };
}

impl_div!(Div, div, Self);
impl_div!(Div, div, &Self);

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
            Fraction::new(1, 10) * Fraction::new(4, 12),
            Fraction::new(4, 120)
        );
        assert_eq!(
            Fraction::new(1, 3) * Fraction::new(4, 12),
            Fraction::new(4, 36)
        );
        assert_eq!(
            Fraction::new(5, 12) * Fraction::new(4, 12),
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
