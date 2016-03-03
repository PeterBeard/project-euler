// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 2^1000?
#![feature(test)]
extern crate test;

/// Calculate a very large exponent
///
/// Only works for fairly small bases
fn large_pow(base: u32, exponent: u32) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let num_digits = number_of_digits(base, exponent);

    // Note that this is a little-endian base-10 number
    digits.push(1);
    for _ in 0..num_digits-1 {
        digits.push(0);
    }

    // Multiply by base exponent times
    for _ in 0..exponent {
        let mut carry = false;
        for i in 0..digits.len() {
            let d = (digits[i] as u32 * base) as u8;

            digits[i] = if carry { d % 10 + 1 } else { d % 10 };
            carry = d > 9;
        }
    }
    digits
}

/// Get the number of digits of a large number expressible as b^e
/// log10(b^e) = e * log10(b)
fn number_of_digits(base: u32, exponent: u32) -> u32 {
    (exponent as f32 * (base as f32).log10()) as u32 + 1
}

/// Add up the digits of a large number
fn sum_of_digits(digits: Vec<u8>) -> u32 {
    let mut sum = 0;
    for d in digits {
        sum += d as u32;
    }
    sum
}

pub fn solution() -> u32 {
    let digits = large_pow(2, 1000);
    sum_of_digits(digits)
}

fn main() {
    println!("The sum of the digits of 2^1000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(1366, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
