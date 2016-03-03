// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// We shall say that an n-digit number is pandigital if it makes use of all the
// digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1
// through 5 pandigital.
// 
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
// multiplicand, multiplier, and product is 1 through 9 pandigital.
// 
// Find the sum of all products whose multiplicand/multiplier/product identity
// can be written as a 1 through 9 pandigital.
//
// HINT: Some products can be obtained in more than one way so be sure to only
// include it once in your sum.
#![feature(test)]
extern crate test;

/// Determine whether the product a * b = c is pandigital
fn is_pandigital_product(a: u32, b: u32) -> bool {
    let mut shrinking_a = a;
    let mut shrinking_b = b;
    let mut c = a * b;
    let mut digit_count: [u32; 10] = [0; 10];
    // Any product bigger than 10^8 must have zeros or repeated digits
    if c > 100000000 {
        return false;
    }

    // Check off the digits as we find them and terminate if we find more than 1
    while shrinking_a >= 1 {
        let digit = (shrinking_a % 10) as usize;
        if digit == 0 || digit_count[digit] > 0 {
            return false;
        }
        digit_count[digit] += 1;
        shrinking_a /= 10;
    }
    while shrinking_b >= 1 {
        let digit = (shrinking_b % 10) as usize;
        if digit == 0 || digit_count[digit] > 0 {
            return false;
        }
        digit_count[digit] += 1;
        shrinking_b /= 10;
    }
    while c >= 1 {
        let digit = (c % 10) as usize;
        if digit == 0 || digit_count[digit] > 0 {
            return false;
        }
        digit_count[digit] += 1;
        c /= 10;
    }

    // If any digits weren't found exactly once then the product fails
    for d in 1..10 {
        if digit_count[d] != 1 {
            return false;
        }
    }
    true
}

pub fn solution() -> u32 {
    let mut products: Vec<u32> = Vec::new();

    // 4999 is a good upper bound since it's the largest integer that results in
    // a sequence with less than 10 digits
    for a in 2..5000 {
        for b in 2..a {
            if is_pandigital_product(a, b) {
                products.push(a*b);
            }
        }
    }

    // Only count each product once
    products.sort();
    products.dedup();
    products.iter().fold(0, |s, v| s + v)
}

fn main() {
    println!("The sum of all pandigital products is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(45228, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
