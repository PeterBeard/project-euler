// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The number 3797 has an interesting property. Being prime itself, it is possible to continuously
// remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7.
// Similarly we can work from right to left: 3797, 379, 37, and 3.
// 
// Find the sum of the only eleven primes that are both truncatable from left to right and right to
// left.
// 
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::primes_upto;

/// Remove the leftmost digit from n
fn truncate_l(n: u32) -> u32 {
    let mut den = 1;
    while n/den > 10 {
        den *= 10;
    }
    n % den
}

/// Remove the rightmost digit from n
fn truncate_r(n: u32) -> u32 {
    n / 10
}

/// Determine whether a number is a truncatable prime
fn is_truncatable_prime(n: u32, primes: &[bool]) -> bool {
    // Values must end with 3 or 7
    if n % 10 != 3 && n % 10 != 7 {
        return false;
    }
    // Similarly, if i isn't prime, we don't have to check anything else
    if !primes[n as usize] {
        return false;
    }
    let (mut l, mut r) = (truncate_l(n), truncate_r(n));
    while l > 0 && r > 0 && primes[l as usize] && primes[r as usize] {
        l = truncate_l(l);
        r = truncate_r(r);
    }
    l == 0 && r == 0
}

pub fn solution() -> u32 {
    // We won't find any truncatable primes higher than this 
    const P_MAX: u32 = 999983;
    let primes = primes_upto(P_MAX);
    let mut sum = 0;
    // 23 is the first possible truncatable prime since 1 isn't
    // prime and neither are 20 or 22 and we don't count 2-7
    for n in 23..P_MAX {
        if is_truncatable_prime(n, &primes) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all truncatable primes is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(748317, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
