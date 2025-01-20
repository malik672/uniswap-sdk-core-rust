use crate::prelude::{BigDecimal, BigInt, BigUint};
use alloy_primitives::{Signed, Uint};
use fastnum::decimal::{Context, Sign};

pub trait ToBig: Sized {
    fn to_big_uint(self) -> BigUint;

    #[inline]
    fn to_big_int(self) -> BigInt {
        self.to_big_uint().to_big_int()
    }

    #[inline]
    fn to_big_decimal(self) -> BigDecimal {
        let x = self.to_big_int();
        BigDecimal::from_parts(
            x.to_bits(),
            0,
            match x.is_negative() {
                false => Sign::Plus,
                true => Sign::Minus,
            },
            Context::default(),
        )
    }
}

impl ToBig for BigInt {
    #[inline]
    fn to_big_uint(self) -> BigUint {
        self.to_bits()
    }

    #[inline]
    fn to_big_int(self) -> BigInt {
        self
    }
}

impl ToBig for BigUint {
    #[inline]
    fn to_big_uint(self) -> BigUint {
        self
    }

    #[inline]
    fn to_big_int(self) -> BigInt {
        BigInt::from_bits(self)
    }
}

impl ToBig for BigDecimal {
    #[inline]
    fn to_big_uint(self) -> BigUint {
        self.round(0).digits()
    }

    #[inline]
    fn to_big_int(self) -> BigInt {
        let res = self.to_big_uint().to_big_int();
        match self.is_negative() {
            true => -res,
            false => res,
        }
    }

    #[inline]
    fn to_big_decimal(self) -> BigDecimal {
        self
    }
}

impl<const BITS: usize, const LIMBS: usize> ToBig for Uint<BITS, LIMBS> {
    #[inline]
    fn to_big_uint(self) -> BigUint {
        BigUint::from_le_slice(&self.as_le_bytes()).unwrap()
    }
}

impl<const BITS: usize, const LIMBS: usize> ToBig for Signed<BITS, LIMBS> {
    #[inline]
    fn to_big_uint(self) -> BigUint {
        self.into_raw().to_big_uint()
    }

    #[inline]
    fn to_big_int(self) -> BigInt {
        BigInt::from_le_slice(&self.into_raw().as_le_bytes()).unwrap()
    }
}

pub trait FromBig: Sized {
    fn from_big_uint(x: BigUint) -> Self;

    #[inline]
    #[must_use]
    fn from_big_int(x: BigInt) -> Self {
        Self::from_big_uint(x.to_bits())
    }
}

impl<const BITS: usize, const LIMBS: usize> FromBig for Uint<BITS, LIMBS> {
    #[inline]
    fn from_big_uint(x: BigUint) -> Self {
        Self::from_limbs_slice(&x.digits()[..LIMBS])
    }
}

impl<const BITS: usize, const LIMBS: usize> FromBig for Signed<BITS, LIMBS> {
    #[inline]
    fn from_big_uint(x: BigUint) -> Self {
        Self::from_raw(Uint::from_big_uint(x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{I256, U256};
    use fastnum::{dec512, i512, u512};

    #[test]
    fn test_uint_to_big() {
        let x = U256::from_limbs([1, 2, 3, 4]);
        let y = BigUint::from(1_u64)
            + (BigUint::from(2_u64) << 64)
            + (BigUint::from(3_u64) << 128)
            + (BigUint::from(4_u64) << 192);
        assert_eq!(x.to_big_uint(), y);
        assert_eq!(x.to_big_int(), y.to_big_int());
        assert_eq!(x.to_big_decimal(), y.to_big_decimal());

        let x = -x;
        let z = (BigUint::from(1_u64) << 256) - y;
        assert_eq!(x.to_big_uint(), z);
        assert_eq!(x.to_big_int(), z.to_big_int());
        assert_eq!(x.to_big_decimal(), z.to_big_decimal());
    }

    #[test]
    fn test_signed_to_big() {
        let x = I256::from_raw(U256::from_limbs([1, 2, 3, 4]));
        let y: BigInt = BigInt::from(1)
            + (BigInt::from(2) << 64)
            + (BigInt::from(3) << 128)
            + (BigInt::from(4) << 192);
        assert_eq!(x.to_big_uint(), y.to_big_uint());
        assert_eq!(x.to_big_int(), y);
        assert_eq!(x.to_big_decimal(), y.to_big_decimal());

        let x = -x;
        let z: BigInt = (BigInt::from(1) << 256) - y;
        assert_eq!(x.to_big_uint(), z.to_big_uint());
        assert_eq!(x.to_big_int(), -y);
        assert_eq!(x.to_big_decimal(), (-y).to_big_decimal());
    }

    #[test]
    fn test_uint_from_big() {
        let x = U256::from_limbs([1, 2, 3, 4]);
        assert_eq!(U256::from_big_uint(x.to_big_uint()), x);
        assert_eq!(U256::from_big_int(x.to_big_int()), x);

        let x = -x;
        assert_eq!(U256::from_big_uint(x.to_big_uint()), x);
        assert_eq!(U256::from_big_int(x.to_big_int()), x);

        assert_eq!(
            U256::from_big_uint(BigInt::from(-1).to_big_uint()),
            U256::MAX
        );
        assert_eq!(U256::from_big_int(BigInt::from(-1)), U256::MAX);

        assert_eq!(
            U256::from_big_uint(I256::MIN.to_big_uint()),
            I256::MIN.into_raw()
        );
        assert_eq!(
            U256::from_big_int(I256::MIN.to_big_int()),
            I256::MIN.into_raw()
        );
    }

    #[test]
    fn test_signed_from_big() {
        let x = I256::from_raw(U256::from_limbs([1, 2, 3, 4]));
        assert_eq!(I256::from_big_uint(x.to_big_uint()), x);
        assert_eq!(I256::from_big_int(x.to_big_int()), x);

        let x = -x;
        assert_eq!(I256::from_big_uint(x.to_big_uint()), x);
        assert_eq!(I256::from_big_int(x.to_big_int()), x);
        assert_eq!(
            x.to_big_uint() + (-x.to_big_int()).to_big_uint(),
            BigUint::from(1_u64) << 256
        );
    }

    #[test]
    fn test_decimal_to_big() {
        let x = dec512!(123456789012345678901234567890.123456789);
        let y = u512!(123456789012345678901234567890);
        let z = i512!(123456789012345678901234567890);
        assert_eq!(x.to_big_uint(), y);
        assert_eq!(x.to_big_int(), z);
        assert_eq!(x.to_big_decimal(), x);

        let x = -x;
        let y = u512!(123456789012345678901234567890);
        let z = i512!(-123456789012345678901234567890);
        assert_eq!(x.to_big_uint(), y);
        assert_eq!(x.to_big_int(), z);
        assert_eq!(x.to_big_decimal(), x);
    }
}
