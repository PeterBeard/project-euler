// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The number, 1406357289, is a 0 to 9 pandigital number because it is made up
// of each of the digits 0 to 9 in some order, but it also has a rather
// interesting sub-string divisibility property.
// 
// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note
// the following:
// 
//     d2d3d4=406 is divisible by 2
//     d3d4d5=063 is divisible by 3
//     d4d5d6=635 is divisible by 5
//     d5d6d7=357 is divisible by 7
//     d6d7d8=572 is divisible by 11
//     d7d8d9=728 is divisible by 13
//     d8d9d10=289 is divisible by 17
// 
// Find the sum of all 0 to 9 pandigital numbers with this property.
#![feature(test)]
extern crate test;

/// Iterate over 0-9 pandigital numbers
struct Pandigital {
    digits: [u64; 10]
}

impl Pandigital {
    fn new() -> Pandigital {
        Pandigital {
            digits: [0,1,2,3,4,5,6,7,8,9]
        }
    }
}

impl Iterator for Pandigital {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let mut next: Vec<u64> = self.digits.to_vec();
        // First we need to find the start of the largest non-increasing suffix
        let mut found_pivot = false;
        let mut pivot = 0;
        let mut swap = 0;
        for i in 1..10 {
            if self.digits[i] > self.digits[i-1] {
                pivot = i-1;
                found_pivot = true;
            }
            if self.digits[i] > self.digits[pivot] {
                swap = i;
            }
        }

        if !found_pivot {
            return None;
        }

        // Work out the value of n
        let mut n = 0;
        let mut val = 1;
        for i in 0..10 {
            n += self.digits[9-i] * val;
            val *= 10;
        }
        // Now we swap the pivot and the rightmost greater value
        next[pivot] = self.digits[swap];
        next[swap] = self.digits[pivot];

        // Now we sort the suffix and we're done
        let mut suffix: Vec<u64> = next.split_off(pivot+1);
        suffix.sort();
        next.append(&mut suffix);
        for i in 0..10 {
            self.digits[i] = next[i];
        }

        Some(n)
    }
}

/// Determine whether a number is substring divisible
fn is_subdivisible(n: u64) -> bool {
    let mut digits: Vec<u64> = Vec::with_capacity(10);
    let mut value = 1;
    for _ in 1..11 {
        digits.push((n / value) % 10);
        value *= 10;
    }
    digits.reverse();

    let primes: [u64; 7] = [2, 3, 5, 7, 11, 13, 17];
    // Test substrings for divisibility
    for offset in 1..8 {
        let value = digits[offset]*100 + digits[offset + 1]*10 + digits[offset + 2];
        if value % primes[offset - 1] != 0 {
            return false;
        }
    }
    true
}

pub fn solution() -> u64 {
    let mut sum = 0;
    for n in Pandigital::new() {
        if is_subdivisible(n) {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of all subdivisible 0-9 pandigital numbers is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(16695334890, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
