// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
// 
// A perfect number is a number for which the sum of its proper divisors is
// exactly equal to the number. For example, the sum of the proper divisors
// of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect
// number.
// 
// A number n is called deficient if the sum of its proper divisors is less than
// n and it is called abundant if this sum exceeds n.
// 
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest
// number that can be written as the sum of two abundant numbers is 24. By
// mathematical analysis, it can be shown that all integers greater than 28123
// can be written as the sum of two abundant numbers. However, this upper limit
// cannot be reduced any further by analysis even though it is known that the
// greatest number that cannot be expressed as the sum of two abundant numbers
// is less than this limit.
// 
// Find the sum of all the positive integers which cannot be written as the sum
// of two abundant numbers.
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::sum_of_divisors;

pub fn solution() -> u32 {
    let mut is_abundant: Vec<bool> = vec![false; 20162];
    for n in 12..is_abundant.len() {
        if sum_of_divisors(n as u32) > n as u32 {
            is_abundant[n as usize] = true;
        }
    }
    
    // We know that all positive integers < 24 can't be expressed as the sum of
    // 2 abundant numbers; their sum is 276
    let mut sum = 276;
    for n in 24..is_abundant.len() {
        // Find two abundant numbers that add to n
        let mut has_sum = false;
        for a in 12..n {
            if is_abundant[a] && is_abundant[n-a] {
                has_sum = true;
                break;
            }
        }
        if !has_sum {
            sum += n;
        }
    }
    sum as u32
}

fn main() {
    println!("The sum of all positive integers that cannot be expressed as the sum of two abundant numbers is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(4179871, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
