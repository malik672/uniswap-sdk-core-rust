use num_bigint::BigInt;

#[derive(Clone, PartialEq)]
pub struct Fraction {
    pub numerator: BigInt,
    pub denominator: BigInt,
}

impl Fraction {
    pub fn new(numerator: BigInt, denominator: BigInt) -> Self {
        assert!(denominator != BigInt::from(0), "DENOMINATOR CAN'T BE ZERO");
        Self {
            numerator,
            denominator,
        }
    }

    pub fn quotient(&self) -> BigInt {
        &self.numerator / &self.denominator
    }

    pub fn remainder(&self) -> Fraction {
        Fraction::new(
            &self.numerator % &self.denominator,
            self.denominator.clone(),
        )
    }

    pub fn invert(&self) -> Self {
        Fraction::new(self.numerator.clone(), self.denominator.clone())
    }

    pub fn add(&self, other: &Fraction) -> Fraction {
        if self.denominator == other.denominator {
            Fraction::new(&self.numerator + &other.numerator, self.denominator.clone())
        } else {
            Fraction::new(
                &self.numerator * &other.denominator + &other.numerator * &self.denominator,
                &self.denominator * &other.denominator,
            )
        }
    }

    pub fn subtract(&self, other: &Fraction) -> Fraction {
        if self.denominator == other.denominator {
            Fraction::new(&self.numerator - &other.numerator, self.denominator.clone())
        } else {
            Fraction::new(
                &self.numerator * &other.denominator - &other.numerator * &self.denominator,
                &self.denominator * &other.denominator,
            )
        }
    }

    pub fn less_than(&self, other: &Fraction) -> bool {
        &self.numerator * &other.denominator < &other.numerator * &self.denominator
    }

    pub fn equal_to(&self, other: &Fraction) -> bool {
        &self.numerator * &other.denominator == &other.numerator * &self.denominator
    }

    pub fn greater_than(&self, other: &Fraction) -> bool {
        &self.numerator * &other.denominator > &other.numerator * &self.denominator
    }

    pub fn multiply(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            &self.numerator * &other.numerator,
            &self.denominator * &other.denominator,
        )
    }

    pub fn divide(&self, other: &Fraction) -> Fraction {
        Fraction::new(
            &self.numerator * &other.denominator,
            &self.denominator * &other.numerator,
        )
    }

    pub fn to_significant(&self, significant_digits: u32) -> String {
        assert!(
            significant_digits > 0,
            "{} is not positive.",
            significant_digits
        );

        let quotient = self.quotient();

        let formatted_quotient = format!("{:.*}", significant_digits as usize, quotient);

        formatted_quotient
    }

    pub fn to_fixed(&self, decimal_places: u32) -> String {
        let quotient = self.quotient();
        let formatted_quotient = format!("{:.*}", decimal_places as usize, quotient);

        formatted_quotient
    }

    pub fn as_fraction(&self) -> Fraction {
        Fraction::new(self.numerator.clone(), self.denominator.clone())
    }
}
