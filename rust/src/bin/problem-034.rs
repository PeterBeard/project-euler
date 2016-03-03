// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
// 
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// 
// Note: as 1! = 1 and 2! = 2 are not sums they are not included.
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::get_digits;

/// Calculate n!
fn factorial(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }

    n * factorial(n-1)
}

/// Find the sum of the factorial of the digits of n
fn sum_of_digits_factorial(n: u32) -> u32 {
    let digits = get_digits(n);
    digits.iter().fold(0, |s, d| s + factorial(d.clone()))
}

pub fn solution() -> u32 {
    let mut sum = 0;
    for n in 10..factorial(9) {
        if sum_of_digits_factorial(n) == n {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all \"curious\" numbers is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(40730, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
