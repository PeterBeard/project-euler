// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Consider all integer combinations of ab for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:
// 
//     2^2=4, 2^3=8, 2^4=16, 2^5=32
//     3^2=9, 3^3=27, 3^4=81, 3^5=243
//     4^2=16, 4^3=64, 4^4=256, 4^5=1024
//     5^2=25, 5^3=125, 5^4=625, 5^5=3125
// 
// If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:
// 
// 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125
// 
// How many distinct terms are in the sequence generated by ab for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?
#![feature(test)]
extern crate test;
extern crate euler_util;
use euler_util::bigint::BigInt;

pub fn solution() -> u32 {
    let mut results: Vec<BigInt> = Vec::new();

    for a in 2..101 {
        let mut value = BigInt::from_u32(a);
        for _ in 2..101 {
            value = value.mul_u32(a);
            results.push(value.clone());
        }
    }

    results.sort();
    results.dedup();
    results.len() as u32
}

fn main() {
    println!("The sequence for a in [2,100] and b in [2,100] has {} terms", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(9183, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
