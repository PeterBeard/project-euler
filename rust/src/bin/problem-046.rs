// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// It was proposed by Christian Goldbach that every odd composite number can be
// written as the sum of a prime and twice a square.
// 
// 9 = 7 + 2×12
// 15 = 7 + 2×22
// 21 = 3 + 2×32
// 25 = 7 + 2×32
// 27 = 19 + 2×22
// 33 = 31 + 2×12
// 
// It turns out that the conjecture was false.
// 
// What is the smallest odd composite that cannot be written as the sum of a
// prime and twice a square?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::prime_sieve;

pub fn solution() -> u32 {
    const LIMIT: u32 = 10000;
    const S_LIMIT: u32 = 50;    // Half the square root of LIMIT

    let mut primes: Vec<u32> = Vec::with_capacity(S_LIMIT as usize);
    let is_prime = prime_sieve(LIMIT);
    for n in 1..LIMIT {
        if is_prime[n as usize] {
            primes.push(n);
        }
    }

    let mut twice_squares: Vec<u32> = Vec::with_capacity(S_LIMIT as usize);
    for n in 1..S_LIMIT {
        twice_squares.push(2*n*n);
    }
    for n in 35..LIMIT {
        if n % 2 != 1 || is_prime[n as usize] {
            continue;
        }
        let mut found_sum = false;
        // For each square*2, see if we can add a prime to get n
        for s in &twice_squares {
            if s > &n {
                break;
            }
            let diff = n - s;
            // Is the difference prime?
            match primes.binary_search(&diff) {
                Ok(_) => {
                    found_sum = true;
                    break;
                }
                Err(_) => {
                    continue;
                }
            };
        }
        if !found_sum {
            return n;
        }
    }
    0
}

fn main() {
    println!("The smallest odd number that can't be written as the sum of a prime and twice a square is {}",
             solution()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(5777, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
