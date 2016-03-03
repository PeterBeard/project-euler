// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::is_prime;

pub fn solution() -> i64 {
    const VALUE:i64 = 600851475143;
    let max_factor = (VALUE as f64).sqrt() as i64 + 1;

    for n in (2..max_factor).rev() {
        if VALUE % n == 0 && is_prime(n) {
            return n;
        }
    }
    1
}

fn main() {
    println!("Largest prime factor of 600,851,475,143 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(6857, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
