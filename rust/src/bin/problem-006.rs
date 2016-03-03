// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
// 
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
// 
// Hence the difference between the sum of the squares of the first ten
// natural numbers and the square of the sum is 3025 − 385 = 2640.
// 
// Find the difference between the sum of the squares of the first one
// hundred natural numbers and the square of the sum.
#![feature(test)]
extern crate test;

pub fn solution() -> i32 {
    let mut sum_squares:i32 = 0;
    let mut sum:i32 = 0;
    
    for i in 1..101 {
        sum += i;
        sum_squares += i*i;
    }
    (sum_squares - sum*sum).abs()
}

fn main() {
    println!("The difference is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(25164150, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
