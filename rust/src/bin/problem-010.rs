// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::is_prime;

pub fn solution() -> i64 {
    let mut sum = 2;
    for n in 3..2000000 {
        if n % 2 != 0 && is_prime(n) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all primes < 2,000,000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(142913828922, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
