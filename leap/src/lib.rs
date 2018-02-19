//! # Leap
//!
//! Given a year, report if it is a leap year.
//!
//! [Source](http://exercism.io/exercises/rust/leap/readme)

/// Given a year, report if it is a leap year
///
/// A year is a leap year in the Gregorian calendar if:
///
/// - it is evenly divisible by 4
/// - except if it is also evenly divisible by 100
/// - unless it is also evenly divisible by 400
///
/// # Arguments
///
/// - `year` - The year to check.
///
/// # Return
///
/// `bool` of whether or not the given [`year`](fn.is_leap_year.html#arguentsfield.year) is a leap year.
pub fn is_leap_year(year: i32) -> bool {
    let is_divisible = |n| year % n == 0;
    is_divisible(4) && (!is_divisible(100) || is_divisible(400))
}
