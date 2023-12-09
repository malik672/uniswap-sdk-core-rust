use super::fraction::{Fraction, Rounding};
use num_bigint::BigInt;

pub struct Percent {
    fraction: Fraction,
}

impl Percent {
    pub const IS_PERCENT: bool = true;

    pub fn new(fraction: Fraction) -> Self {
        Self { fraction }
    }

    pub fn add(&self, other: &Fraction) -> Percent {
        let fraction = self.fraction.add(other);
        Percent::new(fraction)
    }

    pub fn sub(&self, other: &Fraction) -> Percent {
        let fraction = self.fraction.subtract(other);
        Percent::new(fraction)
    }

    pub fn multiply(&self, other: &Fraction) -> Percent {
        let fraction = self.fraction.multiply(other);
        Percent::new(fraction)
    }
    pub fn divide(&self, other: &Fraction) -> Percent {
        let fraction = self.fraction.divide(other);
        Percent::new(fraction)
    }

    pub fn to_significant(&self, other: &Fraction) -> String {
       let hundred = Fraction::new(BigInt::from(100) , BigInt::from(1));
       let mult = Fraction::multiply(&hundred, other);
       Fraction::to_significant(&mult, 5, Rounding::RoundUp)
    }

    pub fn to_fixed(&self, other: &Fraction) -> String {
        let hundred = Fraction::new(BigInt::from(100) , BigInt::from(1));
        let mult = Fraction::multiply(&hundred, other);
        Fraction::to_fixed(&mult, 2, Rounding::RoundUp)
    }
}

pub fn to_percent(fraction: Fraction) -> Percent {
    Percent::new(fraction)
}
