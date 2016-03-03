// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
// see that the 6th prime is 13.
// 
// What is the 10 001st prime number?
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::is_prime;

pub fn solution() -> i64 {
    let mut primes_seen = 2;
    let mut n = 3;

    while primes_seen < 10001 {
        n += 2;
        if is_prime(n) {
            primes_seen += 1;
        }
    }
    n
}

fn main() {
    println!("The 10,001st prime is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(104743, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
