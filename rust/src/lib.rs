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

/// Count the number of digits in a number
pub fn count_digits(n: u32) -> u32 {
    // This is about 10x faster than calculating log10(n)
    let mut count = 0;
    let mut step = 1;
    while step <= n {
        count += 1;
        step *= 10;
    }
    count
}

/// Find all of the prime numbers in [0, n)
pub fn primes_upto(n: u32) -> Vec<bool> {
    let mut primes: Vec<bool> = vec![true; n as usize];
    primes[0] = false;
    primes[1] = false;
    for p in 2..n {
        if primes[p as usize] {
            // If n is prime, all multiples of n are composite
            let mut q = 2*p;
            while q < n {
                primes[q as usize] = false;
                q += p;
            }
        }
    }
    primes
}

/// Determine whether a number is k..l pandigital
pub fn is_pandigital(n: u32, l: u32, k: u32) -> bool {
    if l == k || k < l {
        return false;
    }
    let mut digits: Vec<bool> = vec![false; (k + 1) as usize];
    let mut value = 1;
    println!("{}", n);
    for _ in l..(k + 1) {
        let d = (n % (10*value))/value;
        println!("  {}", d);
        if d > k {
            return false;
        }
        digits[d as usize] = true;
        value *= 10;
    }
    println!("{:#?}", digits);
    for i in l..(k + 1) {
        if !digits[i as usize] {
            return false;
        }
    }
    for i in 0..l {
        if digits[i as usize] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

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

    #[test]
    fn test_count_digits() {
        assert_eq!(2, count_digits(10));
        assert_eq!(5, count_digits(12345));
    }

    #[test]
    fn test_primes_upto() {
        let primes = vec![false, false, true, true, false, true, false, true, false, false];
        assert_eq!(primes, primes_upto(10));
    }

    #[test]
    fn test_is_pandigital() {
        assert_eq!(false, is_pandigital(12345678, 1, 9));
        assert_eq!(false, is_pandigital(1234567890, 1, 9));
        assert!(is_pandigital(1234, 1, 4));
        assert!(is_pandigital(3456, 3, 6));
        assert_eq!(false, is_pandigital(1234, 1, 1));
        assert_eq!(false, is_pandigital(1234, 4, 1));
    }

    #[bench]
    fn bench_is_prime_composite(b: &mut Bencher) {
        let n = black_box(12346);
        b.iter(|| is_prime(n));
    }

    #[bench]
    fn bench_is_prime_prime(b: &mut Bencher) {
        let n = black_box(12347);
        b.iter(|| is_prime(n));
    }

    #[bench]
    fn bench_get_digits(b: &mut Bencher) {
        b.iter(|| get_digits(12345));
    }

    #[bench]
    fn bench_count_digits(b: &mut Bencher) {
        let n = black_box(12345);
        b.iter(|| count_digits(n));
    }

    #[bench]
    fn bench_primes_upto(b: &mut Bencher) {
        b.iter(|| primes_upto(12345));
    }

    #[bench]
    fn bench_is_pandigital(b: &mut Bencher) {
        b.iter(|| is_pandigital(12345, 1, 5));
    }
}
