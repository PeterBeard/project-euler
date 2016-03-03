// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Surprisingly there are only three numbers that can be written as the sum of
// fourth powers of their digits:
// 
//     1634 = 1^4 + 6^4 + 3^4 + 4^4
//     8208 = 8^4 + 2^4 + 0^4 + 8^4
//     9474 = 9^4 + 4^4 + 7^4 + 4^4
// 
// As 1 = 1^4 is not a sum it is not included.
// 
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
// 
// Find the sum of all the numbers that can be written as the sum of fifth
// powers of their digits.
#![feature(test)]
extern crate test;

/// Calculate the sum of the fifth powers of the digits of n
fn sum_of_digits_5(n: u32) -> u32 {
    let mut sum = 0;
    let mut shrinking_n = n;
    while shrinking_n >= 1 {
        let digit = shrinking_n % 10;
        sum += digit.pow(5);
        shrinking_n /= 10;
    }
    sum
}

pub fn solution() -> u32 {
    let mut sum = 0;
    for n in 2..5*9u32.pow(5) {
        if n == sum_of_digits_5(n) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all numbers that can be written as the sum of the 5th powers of their digits is {}",
             solution()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(443839, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
