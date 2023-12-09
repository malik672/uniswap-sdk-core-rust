// use super::{
//     currency_amount::CurrencyAmount,
//     fraction::{Fraction, Rounding},
// };
// use num_bigint::BigInt;

// pub trait Currency {
//     fn decimals(&self) -> u32;
// }

// #[derive(Clone, PartialEq)]

// pub struct Price<TBase, TQuote> {
//     pub base_currency: TBase,
//     pub quote_currency: TQuote,
//     pub numerator: BigInt,
//     pub denominator: BigInt,
//     pub scalar: Fraction,
// }

// impl<TBase: Currency, TQuote: Currency> Price<TBase, TQuote> {
//     pub fn new(
//         base_currency: TBase,
//         quote_currency: TQuote,
//         numerator: BigInt,
//         denominator: BigInt,
//     ) -> Self {
//         let scalar = Fraction::new(
//             BigInt::from(10).pow(base_currency.decimals() as u32),
//             BigInt::from(10).pow(quote_currency.decimals() as u32),
//         );

//         Self {
//             base_currency,
//             quote_currency,
//             numerator,
//             denominator,
//             scalar,
//         }
//     }

//     /**
//      * Flip the price, switching the base and quote currency
//      */
//     pub fn invert(self) -> Price<TBase, TQuote> {
//         Price::new(
//             self.base_currency,
//             self.quote_currency,
//             self.numerator,
//             self.denominator,
//         )
//     }

//     /**
//      * Multiply the price by another price, returning a new price. The other price must have the same base currency as this price's quote currency
//      * @param other the other price
//      */
//     pub fn multiply<TOtherQuote>(&self, other: Price<TQuote, TOtherQuote>) -> Price<TBase, TOtherQuote> {
//         assert!(self.quote_currency == other.base_currency, "TOKEN");
//         let fract = Fraction::multiply(&self, other);
//         Price::new(self.base_currency.clone(), other.quote_currency.clone(), fract.numerator, fract.denominator)
//     }

//     // Methods toSignificant and toFixed would be implemented here
// }
