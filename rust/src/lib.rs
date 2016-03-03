// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Various functions that seem to get used a lot
#![feature(test)]
extern crate test;

pub mod bigint;

/// Determine whether a number is prime
pub fn is_prime(n: i64) -> bool {
    // Negative numbers, zero, and one are not prime
    if n < 2 {
        return false;
    }
    // We don't need to look at factors > sqrt(n)
    let max_factor = ((n as f64).sqrt() as i64) + 1;
    for f in (2..max_factor).rev() {
        if n % f == 0 {
            return false;
        }
    }
    true
}

/// Calculate the sum of all proper divisors of n
pub fn sum_of_divisors(n: u32) -> u32 {
    if n < 2 {
        return n
    }
    let max_factor = (n as f32).sqrt() as u32 + 1;
    let mut sum = 1;
    for i in 2..max_factor {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    sum
}

/// Split an integer into its digits
pub fn get_digits(n: u32) -> Vec<u32> {
    let mut mut_n = n;
    // 2^32 is only 10 digits
    let mut digits: Vec<u32> = Vec::with_capacity(10);
    while mut_n >= 1 {
        digits.push(mut_n % 10);
        mut_n /= 10;
    }
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn is_2_prime() {
        assert!(is_prime(2));
    }

    #[test]
    fn is_13_prime() {
        assert!(is_prime(13));
    }

    #[test]
    fn is_1_prime() {
        assert_eq!(false, is_prime(1));
    }

    #[test]
    fn is_0_prime() {
        assert_eq!(false, is_prime(0));
    }

    #[test]
    fn sum_of_divisors_0() {
        assert_eq!(0, sum_of_divisors(0));
    }

    #[test]
    fn sum_of_divisors_10() {
        assert_eq!(8, sum_of_divisors(10));
    }

    #[test]
    fn sum_of_divisors_28() {
        assert_eq!(28, sum_of_divisors(28));
    }

    #[test]
    fn test_get_digits() {
        let n = 12345;
        let digits = vec![1,2,3,4,5];

        assert_eq!(digits, get_digits(n));
    }

    #[bench]
    fn bench_get_digits(b: &mut Bencher) {
        b.iter(|| get_digits(12345));
    }
}
