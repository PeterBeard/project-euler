// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Let d(n) be defined as the sum of proper divisors of n (numbers less than n
// which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair
// and each of a and b are called amicable numbers.
// 
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44,
// 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4,
// 71 and 142; so d(284) = 220.
// 
// Evaluate the sum of all the amicable numbers under 10000.

/// Calculate the sum of all proper divisors of n
fn d(n: u32) -> u32 {
    // Perfect squares are pretty easy to find
    let sqrt_n = (n as f32).sqrt();
    let max_factor = sqrt_n as u32 + 1;
    if sqrt_n.floor() == sqrt_n.ceil()
    {
        return max_factor;
    }

    let mut sum = 1;
    for i in 2..max_factor {
        if n % i == 0 {
            sum += i;
            sum += n / i;
        }
    }
    sum
}

fn main() {
    println!("d(16) = {}", d(16));
    let search_limit = 10000;
    let mut sum = 0;
    for n in 220..search_limit {
        let dn = d(n);
        // One of the numbers will always be bigger, so there's no need to
        // start searching before m = n + 1. If n's partner is smaller,
        // we will have already found it.
        for m in n+1..search_limit {
            if dn == m && d(m) == n {
                sum += m;
                sum += n;
                break;
            }
        }
    }

    println!("The sum of all amicable numbers under 10000 is {}", sum);
}
