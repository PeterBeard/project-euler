// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
// see that the 6th prime is 13.
// 
// What is the 10 001st prime number?
extern crate euler_util;
use euler_util::is_prime;

fn main() {
    let mut primes_seen = 0;
    let mut n = 1;

    while primes_seen < 10001 {
        n += 1;
        if is_prime(n) {
            primes_seen += 1;
        }
    }
    println!("The 10,001st prime is {}", n);
}
