// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.
extern crate euler_util;
use euler_util::is_prime;

fn main() {
    let mut sum = 2;
    for n in 3..2000000 {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("The sum of all primes < 2,000,000 is {}", sum);
}
