use crate::prelude::*;

pub fn compute_price_impact<TBase: CurrencyTrait, TQuote: CurrencyTrait>(
    mid_price: Price<TBase, TQuote>,
    input_amount: CurrencyAmount<TBase>,
    output_amount: CurrencyAmount<TQuote>,
) -> Percent {
    let quoted_output_amount = mid_price.quote(input_amount);
    // calculate price impact := (exactQuote - outputAmount) / exactQuote
    let price_impact = quoted_output_amount
        .subtract(&output_amount)
        .divide(&quoted_output_amount);
    Percent::new(
        price_impact.numerator().clone(),
        price_impact.denominator().clone(),
    )
}

#[cfg(test)]

mod tests {
    use crate::token;

    use super::*;

    #[test]
    fn test_compute_price_impact() {
        let address_zero = "0x0000000000000000000000000000000000000000";
        let address_one = "0x0000000000000000000000000000000000000001";

        let token = token!(4, address_zero, 25, "Test", "Te");
        let token_1 = token!(3, address_one, 25, "Test", "Te");

        //is correct for zero
        assert!(
            compute_price_impact(
                Price::new(Ether::on_chain(1), token.clone(), 10, 100),
                CurrencyAmount::from_raw_amount(Ether::on_chain(1), 10),
                CurrencyAmount::from_raw_amount(token.clone(), 100)
            ) == Percent::new(0, 10000),
        );

        //is correct for half output
        assert!(
            compute_price_impact(
                Price::new(token.clone(), token_1.clone(), 10, 100),
                CurrencyAmount::from_raw_amount(token.clone(), 10),
                CurrencyAmount::from_raw_amount(token_1.clone(), 50)
            ) == Percent::new(5000, 10000),
        );

        //is negative for more output
        assert!(
            compute_price_impact(
                Price::new(token.clone(), token_1.clone(), 10, 100),
                CurrencyAmount::from_raw_amount(token.clone(), 10),
                CurrencyAmount::from_raw_amount(token_1.clone(), 200)
            ) == Percent::new(-10000, 10000)
        )
    }
}
