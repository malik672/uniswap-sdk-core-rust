/// This module represents currency, including the currency type and the amount as a fraction, also
/// including. methods for arithmetic operations, conversions to and from string representations,
/// and checks for validity.
pub mod currency_amount;
/// This module contains with precise division results, avoiding floating-point arithmetic issues,
/// also including operations like addition, subtraction, multiplication, and division, as well as
/// conversions to other formats.
pub mod fraction;
/// A module represents a percentage as a fraction, with methods for arithmetic and string
/// conversions, it also includes methods for precise calculations involving percentages and ensures
/// accurate representation of percentage values.
pub mod percent;
/// A module represents' a price as a ratio between two currencies, with methods for arithmetic and
/// string conversions.
pub mod price;
