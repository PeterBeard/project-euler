// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The prime 41, can be written as the sum of six consecutive primes:
// 41 = 2 + 3 + 5 + 7 + 11 + 13
// 
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
// 
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.
// 
// Which prime, below one-million, can be written as the sum of the most consecutive primes?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::{is_prime, primes_upto};

pub fn solution() -> u32 {
    const P_MAX: u32 = 1_000_000;
    let primes = primes_upto(P_MAX);
    // Add up the primes until the sum is > P_MAX
    let mut series: Vec<u32> = Vec::with_capacity(primes.len());
    let mut sum = 0;
    for i in 0..primes.len() {
        sum += primes[i];
        if P_MAX < sum {
            break;
        }
        series.push(primes[i]);
    }
    // Remove the smallest value until the sum is prime
    let mut last_prime = series.len();
    loop {
        series.remove(0);
        let sum = series.iter().fold(0, |s, v| s + v);
        if is_prime(sum as i64) {
            return sum;
        }
        // Can we add another prime on the end?
        if sum + primes[last_prime] < P_MAX {
            series.push(primes[last_prime]);
            last_prime += 1;
        }
    }
}

fn main() {
    println!("The prime under 1,000,000 expressible as the sum of the most consecutive primes is {}",
             solution()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(997651, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
