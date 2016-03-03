// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// If we list all the natural numbers below 10 that are multiples of 3 or 5,
// we get 3, 5, 6 and 9. The sum of these multiples is 23.
//
// Find the sum of all the multiples of 3 or 5 below 1000. 
#![feature(test)]
extern crate test;

pub fn solution() -> u32 {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum += n;
        }
    }
    sum
}

fn main() {
    println!("The sum of the multiples of 3 or 5 below 1000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(233168, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
