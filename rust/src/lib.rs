// Various functions that seem to get used a lot

// Determine whether a number is prime
pub fn is_prime(n: i64) -> bool {
    // We don't need to look at factors > sqrt(n)
    let max_factor = ((n as f64).sqrt() as i64) + 1;
    for f in (2..max_factor).rev() {
        if n % f == 0 {
            return false;
        }
    }
    true
}
