// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can
// see that the 6th prime is 13.
// 
// What is the 10 001st prime number?

// Determine whether a number is prime
fn is_prime(n: i32) -> bool {
    // We don't need to look at factors > sqrt(n)
    let max_factor = ((n as f32).sqrt() as i32) + 1;
    for f in (2..max_factor).rev() {
        if n % f == 0 {
            return false;
        }
    }
    true
}

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
