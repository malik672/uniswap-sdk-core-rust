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
            x.unsigned_abs(),
            0,
            if x.is_negative() {
                Sign::Minus
            } else {
                Sign::Plus
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

    /// Construct BigUint from U256 limbs (little-endian)
    fn biguint_from_limbs(limbs: [u64; 4]) -> BigUint {
        BigUint::from(limbs[0])
            + (BigUint::from(limbs[1]) << 64)
            + (BigUint::from(limbs[2]) << 128)
            + (BigUint::from(limbs[3]) << 192)
    }

    /// Construct BigInt from limbs as unsigned value (little-endian)
    fn bigint_from_limbs_unsigned(limbs: [u64; 4]) -> BigInt {
        BigInt::from_bits(biguint_from_limbs(limbs))
    }

    /// Construct BigInt from I256 limbs using two's complement (little-endian)
    fn bigint_from_signed_limbs(limbs: [u64; 4]) -> BigInt {
        let raw = bigint_from_limbs_unsigned(limbs);
        // Check if high bit is set (negative in two's complement)
        if limbs[3] & 0x8000_0000_0000_0000 != 0 {
            // Convert from two's complement: subtract 2^256
            raw - (BigInt::from(1) << 256)
        } else {
            raw
        }
    }

    #[test]
    fn test_uint_to_big() {
        let x = U256::from_limbs([1, 2, 3, 4]);
        let y = biguint_from_limbs([1, 2, 3, 4]);
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
        let x = I256::from_limbs([1, 2, 3, 4]);
        let y = bigint_from_signed_limbs([1, 2, 3, 4]);
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
        let x = I256::from_limbs([1, 2, 3, 4]);
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

    #[test]
    fn test_big_to_decimal() {
        let u = u512!(12345678901234567890123456789012345678901234567890123456789012345678);
        let d = dec512!(12345678901234567890123456789012345678901234567890123456789012345678);
        assert_eq!(u.to_big_decimal(), d);

        let i = i512!(-12345678901234567890123456789012345678901234567890123456789012345678);
        let d = -d;
        assert_eq!(i.to_big_decimal(), d);
    }

    #[test]
    fn test_zero() {
        let x = U256::ZERO;
        assert_eq!(x.to_big_uint(), BigUint::from(0_u64));
        assert_eq!(x.to_big_int(), BigInt::from(0));
        assert_eq!(U256::from_big_uint(x.to_big_uint()), x);
    }

    #[test]
    fn test_max() {
        let x = U256::MAX;
        let expected = (BigUint::from(1_u64) << 256) - BigUint::from(1_u64);
        assert_eq!(x.to_big_uint(), expected);
        assert_eq!(U256::from_big_uint(x.to_big_uint()), x);
    }

    #[test]
    fn test_signed_boundaries() {
        // I256::MIN has high bit set: [0, 0, 0, 0x8000_0000_0000_0000]
        let x = I256::from_limbs([0, 0, 0, 0x8000_0000_0000_0000]);
        assert_eq!(x, I256::MIN);
        assert_eq!(I256::from_big_int(x.to_big_int()), x);

        // I256::MAX: [MAX, MAX, MAX, 0x7FFF_FFFF_FFFF_FFFF]
        let y = I256::from_limbs([u64::MAX, u64::MAX, u64::MAX, 0x7FFF_FFFF_FFFF_FFFF]);
        assert_eq!(y, I256::MAX);
        assert_eq!(I256::from_big_int(y.to_big_int()), y);
    }

    #[test]
    fn test_negative_one() {
        let neg_one = I256::MINUS_ONE;
        assert_eq!(neg_one.to_big_int(), BigInt::from(-1));
        assert_eq!(I256::from_big_int(neg_one.to_big_int()), neg_one);
    }

    // Property-based tests using proptest
    mod proptests {
        use super::*;
        use proptest::prelude::*;

        proptest! {
            #[test]
            fn prop_u256_to_big_uint(limbs in prop::array::uniform4(any::<u64>())) {
                let x = U256::from_limbs(limbs);
                let expected = biguint_from_limbs(limbs);
                prop_assert_eq!(x.to_big_uint(), expected);
            }

            #[test]
            fn prop_u256_to_big_int(limbs in prop::array::uniform4(any::<u64>())) {
                let x = U256::from_limbs(limbs);
                let expected = bigint_from_limbs_unsigned(limbs);
                prop_assert_eq!(x.to_big_int(), expected);
            }

            #[test]
            fn prop_u256_to_big_decimal(limbs in prop::array::uniform4(any::<u64>())) {
                let x = U256::from_limbs(limbs);
                let expected = biguint_from_limbs(limbs).to_big_decimal();
                prop_assert_eq!(x.to_big_decimal(), expected);
            }

            #[test]
            fn prop_i256_to_big_int(limbs in prop::array::uniform4(any::<u64>())) {
                let x = I256::from_limbs(limbs);
                let expected = bigint_from_signed_limbs(limbs);
                prop_assert_eq!(x.to_big_int(), expected);
            }

            #[test]
            fn prop_i256_to_big_uint(limbs in prop::array::uniform4(any::<u64>())) {
                let x = I256::from_limbs(limbs);
                // I256::to_big_uint() returns the raw underlying U256 bits, not the signed value
                let expected = biguint_from_limbs(limbs);
                prop_assert_eq!(x.to_big_uint(), expected);
            }

            #[test]
            fn prop_i256_to_big_decimal(limbs in prop::array::uniform4(any::<u64>())) {
                let x = I256::from_limbs(limbs);
                let expected = bigint_from_signed_limbs(limbs).to_big_decimal();
                prop_assert_eq!(x.to_big_decimal(), expected);
            }

            // Round-trip tests
            #[test]
            fn prop_u256_roundtrip(limbs in prop::array::uniform4(any::<u64>())) {
                let x = U256::from_limbs(limbs);
                prop_assert_eq!(U256::from_big_uint(x.to_big_uint()), x);
                prop_assert_eq!(U256::from_big_int(x.to_big_int()), x);
            }

            #[test]
            fn prop_i256_roundtrip(limbs in prop::array::uniform4(any::<u64>())) {
                let x = I256::from_limbs(limbs);
                prop_assert_eq!(I256::from_big_uint(x.to_big_uint()), x);
                prop_assert_eq!(I256::from_big_int(x.to_big_int()), x);
            }
        }
    }
}
