// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The first two consecutive numbers to have two distinct prime factors are:
// 
// 14 = 2 × 7
// 15 = 3 × 5
// 
// The first three consecutive numbers to have three distinct prime factors are:
// 
// 644 = 2² × 7 × 23
// 645 = 3 × 5 × 43
// 646 = 2 × 17 × 19.
// 
// Find the first four consecutive integers to have four distinct prime factors.
// What is the first of these numbers?
#![feature(test)]
extern crate test;

/// Find the prime factorization of an integer
/// If a factor is repeated (e.g. 4 = 2^2), then it will be repeated in the vector
fn prime_factorization(n: u32) -> Vec<u32> {
    let mut factors: Vec<u32> = Vec::new();
    let mut shrinking_n = n;
    // Start pulling out prime numbers
    for p in 2..n+1 {
        while shrinking_n % p == 0 {
            shrinking_n /= p;
            factors.push(p);
        }
        if shrinking_n <= 1 {
            break;
        }
    }
    factors
}

pub fn solution() -> u32 {
    const N_FACTORS: u32 = 4;

    let mut count = 0;
    for n in 647..200000 {
        let mut factors = prime_factorization(n);
        factors.dedup();
        if factors.len() == N_FACTORS as usize {
            count += 1;
        } else {
            count = 0;
        }
        if count == N_FACTORS {
            return n-3;
        }
    }
    0
}

fn main() {
    println!("The first in the smallest set of four consecutive integers with four prime factors is {}",
             solution()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(134043, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
