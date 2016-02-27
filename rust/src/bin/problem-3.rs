// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?
extern crate euler_util;
use euler_util::is_prime;

fn main() {
    const VALUE:i64 = 600851475143;
    let max_factor = (VALUE as f64).sqrt() as i64 + 1;

    for n in (2..max_factor).rev() {
        if VALUE % n == 0 && is_prime(n) {
            println!("Largest prime factor of {} is {}", VALUE, n);
            break;
        }
    }
}
