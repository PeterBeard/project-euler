// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is
// also prime.
// 
// What is the largest n-digit pandigital prime that exists?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::{is_pandigital, is_prime};

pub fn solution() -> u32 {
    let mut max = 0;
    for n in (3..10).rev() {
        let mut p = 0;
        for d in 0..(n-1) {
            p += n - d;
            p *= 10;
        }
        p += 1;
        // If any n-digit pandigital number is divisible by 3 (or 9), then all
        // of them will be, i.e. none of them will be prime.
        if p % 3 == 0 {
            continue;
        }

        while p > 10u32.pow(n-1) {
            if p > max && is_pandigital(p, n) && is_prime(p as i64) {
                max = p;
                break;
            }
            p -= 9;
        }
    }
    max
}

fn main() {
    println!("The largest n-digit pandigital prime is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(7652413, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
