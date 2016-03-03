// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// n! means n × (n − 1) × ... × 3 × 2 × 1
// 
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// 
// Find the sum of the digits in the number 100!
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::bigint::{BigInt, sum_of_digits};

pub fn solution() -> u32 {
    sum_of_digits(&BigInt::from_u32(100).fact())

}

fn main() {
    println!("The sum of the digits of 100! is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(648, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
