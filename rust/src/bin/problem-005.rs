// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// 2520 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of
// the numbers from 1 to 20?
#![feature(test)]
extern crate test;

pub fn solution() -> u32 {
    // Any number divisible by 11-20 is also divisible by 2-10
    const FACTORS: [u32; 10] = [20, 19, 18, 17, 16, 15, 14, 13, 12, 11];
    let mut n = 20;
    let mut all_divisible = false;
    // This might take a while; 20! is pretty big...
    while !all_divisible {
        n += 20;
        all_divisible = true;
        for f in FACTORS.iter() {
            if n % f != 0 {
                all_divisible = false;
                break;
            }
        }
    }
    return n;
}

fn main() {
    println!("The smallest positive integer divisible by all of [1,20] is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(232792560, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
