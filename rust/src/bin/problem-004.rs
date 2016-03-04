// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.
#![feature(test)]
extern crate test;

// Determine whether n is a palindrome
fn is_palindrome(n: i32) -> bool {
    // Multiples of 10 can't be palindromes
    if n % 10 == 0 {
        return false;
    }

    let mut den = 1;
    let mut digits = Vec::with_capacity(6);
    while den < n {
        digits.push((n/den) % 10);
        den *= 10;
    }

    for i in 0..digits.len() {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
    }
    true
}

pub fn solution() -> i32 {
    let mut largest = 0;
    for n in 100..1000 {
        for m in 100..1000 {
            let product = n*m;
            if product > largest && is_palindrome(product) {
                largest = product;
            }
        }
    }
    largest
}

fn main() {
    println!("The largest palindrome product of two three-digit numbers is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(906609, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
