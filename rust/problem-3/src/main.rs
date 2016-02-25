// The prime factors of 13195 are 5, 7, 13 and 29.
// 
// What is the largest prime factor of the number 600851475143 ?

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
    const VALUE:i64 = 600851475143;
    let max_factor = (VALUE as f64).sqrt() as i64 + 1;

    for n in (2..max_factor).rev() {
        if VALUE % n == 0 && is_prime(n) {
            println!("Largest prime factor of {} is {}", VALUE, n);
            break;
        }
    }
}
