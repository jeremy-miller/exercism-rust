//! # Collatz Conjecture
//!
//! The Collatz Conjecture or 3x+1 problem can be summarized as follows:
//!
//! Take any positive integer n. If n is even, divide n by 2 to get n / 2. If n is odd, multiply n by 3 and add 1 to get 3n + 1.
//! Repeat the process indefinitely. The conjecture states that no matter which number you start with,
//! you will always reach 1 eventually.
//!
//! Given a number n, return the number of steps required to reach 1.
//!
//! [Source](http://exercism.io/exercises/rust/collatz-conjecture/readme)

/// Given a number `n`, return the number of steps required to reach 1 using the Collatz Conjecture.
///
/// # Arguments
///
/// `n` - Initial number.  Must be positive.
///
/// # Return
///
/// `Result` of either the number of steps to reach 1 using the Collatz Conjecture (starting at [`n`](fn.collatz.html#arguentsfield.n),
/// or an `Err` message.
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        return Err("'n' must be positive");
    }

    let (_, count) = collatz_positive(n, 0);

    return Ok(count);
}

fn collatz_positive(n: u64, count: u64) -> (u64, u64) {
    if n == 1 {
        return (n, count);
    }

    match n % 2 == 0 {
        true => collatz_positive(n / 2, count + 1),
        false => collatz_positive(n * 3 + 1, count + 1)
    }
}
