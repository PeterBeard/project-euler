// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms
// increases by 3330, is unusual in two ways: (i) each of the three terms are
// prime, and, (ii) each of the 4-digit numbers are permutations of one another.
// 
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes,
// exhibiting this property, but there is one other 4-digit increasing sequence.
// 
// What 12-digit number do you form by concatenating the three terms in this sequence?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::{get_digits, is_prime};

pub fn solution() -> u64 {
    // 1489 is the next prime after 1487
    for n in 1489..10000 {
        if !is_prime(n as i64) {
            continue;
        }

        let mut d_n = get_digits(n as u32);
        d_n.sort();

        let max_m = (10000 - n) / 2;
        for m in 1..max_m {
            let p = n + m;
            let q = p + m;

            // Sort the digits so we can easily compare them
            let mut d_p = get_digits(p as u32);
            let mut d_q = get_digits(q as u32);
            d_p.sort();
            d_q.sort();

            if d_n == d_p && d_n == d_q && 
               is_prime(p as i64) && is_prime(q as i64) {
                return n*100000000 + p * 10000 + q;
            }
        }
    }
    0
}

fn main() {
    println!("The concatenation of the second sequence is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(296962999629, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
