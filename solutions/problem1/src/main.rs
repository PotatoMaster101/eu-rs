//! Solution for [problem 1](https://projecteuler.net/problem=1).
#![forbid(unsafe_code, missing_docs)]

/// The maximum number for problem 1.
const MAX: u64 = 999;

/// Returns the sum from 1 to `n`.
///
/// # Arguments
/// - `n`: The last number in the sum.
///
/// # Returns
/// The sum from 1 to `n`.
const fn sum(n: u64) -> u64 {
    (n * (n + 1)) / 2
}

fn main() {
    let div_3 = 3 * sum(MAX / 3);
    let div_5 = 5 * sum(MAX / 5);
    let div_15 = 15 * sum(MAX / 15);
    println!("{}", div_3 + div_5 - div_15);
}
