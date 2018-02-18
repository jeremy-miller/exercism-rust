//! # Gigasecond
//!
//! Calculate the moment when someone has lived for 10<sup>9</sup> seconds.
//!
//! [Source](http://exercism.io/exercises/rust/gigasecond/readme)

extern crate chrono;
extern crate time;

use chrono::prelude::{ DateTime, Utc };
use time::Duration;

/// Calculate the moment when someone has lived for 10<sup>9</sup> seconds.
///
/// # Arguments
///
/// - `start_date` - Initial [`DateTime`](https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html) to use as birthday.
///
/// # Return
///
/// A [`DateTime`](https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html) containing the exact date and time
/// someone has lived for 10<sup>9</sup> seconds based on the [`start_date`](fn.after.html#arguentsfield.start_date).
pub fn after(start_date: DateTime<Utc>) -> DateTime<Utc> {
    start_date + Duration::seconds(1_000_000_000)
}
