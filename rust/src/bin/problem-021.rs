// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Let sum_of_divisors(n) be defined as the sum of proper divisors of n (numbers less than n
// which divide evenly into n).
// If sum_of_divisors(a) = b and sum_of_divisors(b) = a, where a ≠ b, then a and b are an amicable pair
// and each of a and b are called amicable numbers.
// 
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44,
// 55 and 110; therefore sum_of_divisors(220) = 284. The proper divisors of 284 are 1, 2, 4,
// 71 and 142; so sum_of_divisors(284) = 220.
// 
// Evaluate the sum of all the amicable numbers under 10000.
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::sum_of_divisors;

pub fn solution() -> u32 {
    let search_limit = 10000;
    let mut sum = 0;
    for n in 220..search_limit {
        let dn = sum_of_divisors(n);
        // One of the numbers will always be bigger, so there's no need to
        // start searching before m = n + 1. If n's partner is smaller,
        // we will have already found it.
        for m in n+1..search_limit {
            if dn == m && sum_of_divisors(m) == n {
                sum += m;
                sum += n;
                break;
            }
        }
    }
    sum
}

fn main() {
    println!("The sum of all amicable numbers under 10000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(31626, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
