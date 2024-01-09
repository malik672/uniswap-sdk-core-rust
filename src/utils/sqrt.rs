use crate::prelude::*;

/// Computes floor(sqrt(value))
///
/// # Arguments
///
/// * `value`: the value for which to compute the square root, rounded down
///
/// returns: BigInt
///
pub fn sqrt(value: &BigInt) -> Result<BigInt, Error> {
    if !value >= Zero::zero() {
        return Err(Error::Incorrect("NEGATIVE".to_owned()));
    }

    // If the value is less than or equal to MAX_SAFE_INTEGER,
    // we can safely convert it to an i64 and use the primitive sqrt function.
    if let Some(safe_value) = value.to_i64() {
        return Ok(((safe_value as f64).sqrt().floor() as i64)
            .to_bigint()
            .unwrap());
    }

    // Otherwise, we use the Babylonian method to calculate the square root.
    let two = 2.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    let mut z = value.clone();
    let mut x = (value / &two) + &one;

    while x < z {
        z = x.clone();
        x = ((value / &x) + &x) / &two;
    }

    Ok(z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_0_1000() {
        for i in 0..1000 {
            let sqrt_i = sqrt(&BigInt::from(i));
            assert_eq!(
                sqrt_i.unwrap(),
                BigInt::from((i as f64).sqrt().floor() as i64)
            );
        }
    }

    #[test]
    fn test_sqrt_2_powers() {
        for i in 0..256 {
            let root = BigInt::from(2).pow(i as u32);
            let root_squared = &root * &root;
            assert_eq!(sqrt(&root_squared).unwrap(), root);
        }
    }

    #[test]
    fn test_sqrt_max_uint256() {
        let max_uint256_string =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        let max_uint256 = BigInt::from_str_radix(max_uint256_string, 10).unwrap();
        let expected_sqrt =
            BigInt::from_str_radix("340282366920938463463374607431768211455", 10).unwrap();
        assert_eq!(sqrt(&max_uint256).unwrap(), expected_sqrt);
    }
}
