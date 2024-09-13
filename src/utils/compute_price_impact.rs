use crate::prelude::*;

/// Returns the percent difference between the mid price and the execution price, i.e. price impact.
///
/// # Arguments
///
/// * `midPrice`: mid price before the trade
/// * `inputAmount`: the input amount of the trade
/// * `outputAmount`: the output amount of the trade
///
/// returns: Percent
#[inline]
pub fn compute_price_impact<TBase: Currency, TQuote: Currency>(
    mid_price: &Price<TBase, TQuote>,
    input_amount: &CurrencyAmount<TBase>,
    output_amount: &CurrencyAmount<TQuote>,
) -> Result<Percent, Error> {
    let quoted_output_amount = mid_price.quote(input_amount)?;
    // calculate price impact := (exactQuote - outputAmount) / exactQuote
    let price_impact = quoted_output_amount
        .subtract(output_amount)?
        .divide(&quoted_output_amount)?;
    Ok(Percent::new(
        price_impact.numerator,
        price_impact.denominator,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token;

    #[test]
    fn test_compute_price_impact() {
        let address_zero = "0x0000000000000000000000000000000000000000";
        let address_one = "0x0000000000000000000000000000000000000001";

        let token = token!(4, address_zero, 25, "Test", "Te");
        let token_1 = token!(3, address_one, 25, "Test", "Te");

        //is correct for zero
        assert!(
            compute_price_impact(
                &Price::new(Ether::on_chain(1), token.clone(), 10, 100),
                &CurrencyAmount::from_raw_amount(Ether::on_chain(1), 10).unwrap(),
                &CurrencyAmount::from_raw_amount(token.clone(), 100).unwrap()
            )
            .unwrap()
                == Percent::default(),
        );

        //is correct for half output
        assert!(
            compute_price_impact(
                &Price::new(token.clone(), token_1.clone(), 10, 100),
                &CurrencyAmount::from_raw_amount(token.clone(), 10).unwrap(),
                &CurrencyAmount::from_raw_amount(token_1.clone(), 50).unwrap()
            )
            .unwrap()
                == Percent::new(5000, 10000),
        );

        //is negative for more output
        assert_eq!(
            compute_price_impact(
                &Price::new(token.clone(), token_1.clone(), 10, 100),
                &CurrencyAmount::from_raw_amount(token, 10).unwrap(),
                &CurrencyAmount::from_raw_amount(token_1, 200).unwrap()
            )
            .unwrap(),
            Percent::new(-10000, 10000)
        )
    }
}
