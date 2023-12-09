use num_bigint::{BigInt, ToBigInt};
use num_traits::{ToPrimitive, Zero};

const MAX_SAFE_INTEGER: i64 = i64::MAX;

fn sqrt(value: &BigInt) -> BigInt {
    assert!(*value >= Zero::zero(), "NEGATIVE");

    // If the value is less than or equal to MAX_SAFE_INTEGER,
    // we can safely convert it to an i64 and use the primitive sqrt function.
    if let Some(safe_value) = value.to_i64() {
        if safe_value <= MAX_SAFE_INTEGER {
            return ((safe_value as f64).sqrt().floor() as i64)
                .to_bigint()
                .unwrap();
        }
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

    z
}
