// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Various functions that seem to get used a lot
pub mod bigint;

/// Determine whether a number is prime
pub fn is_prime(n: i64) -> bool {
    // We don't need to look at factors > sqrt(n)
    let max_factor = ((n as f64).sqrt() as i64) + 1;
    for f in (2..max_factor).rev() {
        if n % f == 0 {
            return false;
        }
    }
    true
}

/// Calculate the sum of all proper divisors of n
pub fn sum_of_divisors(n: u32) -> u32 {
    let max_factor = (n as f32).sqrt() as u32 + 1;
    let mut sum = 1;
    for i in 2..max_factor {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    sum
}
