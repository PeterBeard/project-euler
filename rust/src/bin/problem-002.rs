// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Each new term in the Fibonacci sequence is generated by adding the
// previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// 
// By considering the terms in the Fibonacci sequence whose values do not
// exceed four million, find the sum of the even-valued terms.
#![feature(test)]
extern crate test;

pub fn solution() -> u32 {
    let (mut a, mut b, mut c) = (1, 2, 3);
    let mut sum = 2;
    while c <= 4000000 {
        c = a + b;
        a = b;
        b = c;

        if c % 2 == 0 {
            sum += c;
        }
    }
    sum
}

fn main() {
    println!("The sum of the Fibonacci numbers < 4,000,000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(4613732, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
