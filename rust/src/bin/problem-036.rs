// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
// 
// Find the sum of all numbers, less than one million, which are palindromic in
// base 10 and base 2.
// 
// (Please note that the palindromic number, in either base, may not include
// leading zeros.)
//
#![feature(test)]
extern crate test;

/// Determine whether a number is a palindrome in bases 2 and 10
fn is_double_palindrome(n: u32) -> bool {
    // Multiples of 10 or 2 won't work
    if n % 2 == 0 || n % 10 == 0 {
        return false;
    }

    // Is this a palindrome in base 10?
    let mut digits = Vec::with_capacity(6);
    let mut den = 1;
    while den < n {
        digits.push((n/den) % 10);
        den *= 10;
    }

    for i in 0..digits.len() {
        if digits[i] != digits[digits.len() - i - 1] {
            return false;
        }
    }

    den = 1;
    let mut bits = Vec::with_capacity(20);
    while den < n {
        bits.push((n/den) % 2);
        den *= 2;
    }
    
    // Is this a palindrome in base 2?
    for i in 0..bits.len() {
        if bits[i] != bits[bits.len() - i - 1] {
            return false;
        }
    }

    true
}

pub fn solution() -> u32 {
    let mut sum = 0;
    for n in 1..1000000 {
        if is_double_palindrome(n) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all base 10 and base 2 palindromes under 1,000,000 is {}",
             solution()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(872187, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
