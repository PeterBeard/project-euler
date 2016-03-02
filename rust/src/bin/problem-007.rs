// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
// see that the 6th prime is 13.
// 
// What is the 10 001st prime number?
extern crate euler_util;
use euler_util::is_prime;

fn main() {
    let mut primes_seen = 2;
    let mut n = 3;

    while primes_seen < 10001 {
        n += 2;
        if is_prime(n) {
            primes_seen += 1;
        }
    }
    println!("The 10,001st prime is {}", n);
}
