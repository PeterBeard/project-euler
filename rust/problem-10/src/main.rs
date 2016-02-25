// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

// Determine whether a number is prime
fn is_prime(n: i64) -> bool {
    // We don't need to look at factors > sqrt(n)
    let max_factor = ((n as f64).sqrt() as i64) + 1;
    for f in (2..max_factor).rev() {
        if n % f == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut sum = 2;
    for n in 3..2000000 {
        if is_prime(n) {
            sum += n;
        }
    }
    println!("The sum of all primes < 2,000,000 is {}", sum);
}
