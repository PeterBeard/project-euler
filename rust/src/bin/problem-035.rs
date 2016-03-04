// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The number, 197, is called a circular prime because all rotations of the
// digits: 197, 971, and 719, are themselves prime.
// 
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71,
// 73, 79, and 97.
// 
// How many circular primes are there below one million?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::{count_digits, primes_upto};

/// Rotate the digits of a u32, e.g. 197 -> 719
fn rotate_number(n: u32) -> u32 {
    if n < 10 {
        return n;
    }
    (n % 10) * 10u32.pow(count_digits(n) - 1) + n / 10
}

/// Determine whether a number is a circular prime
fn is_circular_prime(n: u32, primes: &[bool]) -> bool {
    if !primes[n as usize] {
        return false;
    }
    let mut rotated = rotate_number(n);
    while rotated != n {
        if !primes[rotated as usize] {
            return false;
        }
        rotated = rotate_number(rotated);
    }
    true
}

pub fn solution() -> u32 {
    let primes = primes_upto(1000000);
    let mut count = 0;
    for n in 2..1000000 {
        if is_circular_prime(n, &primes) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("There are {} circular primes below 1,000,000", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(55, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
