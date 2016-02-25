// A palindromic number reads the same both ways. The largest palindrome made
// from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

// Determine whether n is a palindrome
fn is_palindrome(n: i32) -> bool {
    let n_digits = (n as f32).log10() as i32;
    // Numbers divisible by a power of 10 can't be palindromes
    for i in 1..n_digits+1 {
        if n % (10f32.powi(i) as i32) == 0 {
            return false;
        }
    }
    // Reverse the digits of the number
    let mut reverse_n = 0;
    let mut shrinking_n = n;
    for i in (0..n_digits+1).rev() {
        let place_value = 10f32.powi(i) as i32;
        while shrinking_n >= place_value {
            shrinking_n -= place_value;
            reverse_n += 10f32.powi(n_digits - i) as i32;
        }
    }

    // n is a palindrome if it's the same backwards as forwards
    n == reverse_n
}

fn main() {
    let mut largest = 0;
    let mut max_m = 0;
    let mut max_n = 0;
    for n in 100..1000 {
        for m in 100..1000 {
            let product = n*m;
            if product > largest && is_palindrome(product) {
                largest = product;
                max_m = m;
                max_n = n;
            }
        }
    }
    println!("The largest palindrome product of two three-digit numbers is {} ({} * {})",
        largest,
        max_n,
        max_m
    );
}
