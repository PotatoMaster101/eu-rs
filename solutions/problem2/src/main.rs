//! Solution for [problem 2](https://projecteuler.net/problem=2).
#![forbid(unsafe_code, missing_docs)]

/// Adds all the even Fibonacci numbers for the first `n` terms.
///
/// # Arguments
/// - `n`: The count of Fibonacci terms.
///
/// # Returns
/// The sum of all the even Fibonacci numbers for the first `n` terms.
fn even_fib(n: u64) -> u64 {
    let (mut result, mut a, mut b) = (0, 1, 1);
    for _ in 0..n {
        let num = a + b;
        if num % 2 == 0 {
            result += num;
        }
        a = b;
        b = num;
    }
    result
}

fn main() {
    println!("{}", even_fib(33));
}
