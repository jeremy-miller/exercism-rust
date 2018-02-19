//! # Raindrops
//!
//! Convert a number to a string, the contents of which depend on the number's factors.
//!
//! - If the number has 3 as a factor, output 'Pling'.
//! - If the number has 5 as a factor, output 'Plang'.
//! - If the number has 7 as a factor, output 'Plong'.
//! - If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
//!
//! [Source](http://exercism.io/exercises/rust/raindrops/readme)

/// Convert a number to a string, the contents of which depend on the number's factors.
///
/// - If the number has 3 as a factor, output 'Pling'.
/// - If the number has 5 as a factor, output 'Plang'.
/// - If the number has 7 as a factor, output 'Plong'.
/// - If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
///
/// # Arguments
///
/// - `number` - The number to be converted to a string.
///
/// # Return
///
/// `String` whose contents are determined by the factors of [`number`](fn.raindrops.html#arguentsfield.number).
pub fn raindrops(number: i32) -> String {
    let mut string = String::new();
    if number % 3 == 0 {
        string.push_str("Pling");
    }
    if number % 5 == 0 {
        string.push_str("Plang");
    }
    if number % 7 == 0 {
        string.push_str("Plong");
    }
    if string.is_empty() {
        let digits = format!("{}", number);
        string.push_str(&digits);
    }
    string
}
