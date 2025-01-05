use crate::prelude::*;
use fastnum::i512;

/// Computes floor(sqrt(value))
///
/// # Arguments
///
/// * `value`: the value for which to compute the square root, rounded down
///
/// returns: BigInt
#[inline]
pub fn sqrt(value: BigInt) -> Result<BigInt, Error> {
    const ONE: BigInt = i512!(1);
    const TWO: BigInt = i512!(2);
    match value {
        v if v < BigInt::ZERO => Err(Error::Invalid("NEGATIVE")),
        BigInt::ZERO => Ok(BigInt::ZERO),
        v if v <= TWO => Ok(ONE),
        _ => {
            let mut z = value;
            let mut x = (value / TWO) + ONE;
            while x < z {
                z = x;
                x = (value / x + x) / TWO;
            }
            Ok(z)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_0_1000() {
        for i in 0..1000 {
            assert_eq!(
                sqrt(BigInt::from(i)).unwrap(),
                BigInt::from((i as f64).sqrt().floor() as i64)
            );
        }
    }

    #[test]
    fn test_sqrt_2_powers() {
        for i in 0..256 {
            let root = BigInt::from(2).pow(i as u32);
            let root_squared = root * root;
            assert_eq!(sqrt(root_squared).unwrap(), root);
        }
    }

    #[test]
    fn test_sqrt_max_uint256() {
        let expected_sqrt = i512!(340282366920938463463374607431768211455);
        assert_eq!(sqrt(MAX_UINT256).unwrap(), expected_sqrt);
    }
}
