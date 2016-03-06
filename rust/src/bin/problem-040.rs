// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// An irrational decimal fraction is created by concatenating the positive integers:
// 
// 0.123456789101112131415161718192021...
// 
// It can be seen that the 12th digit of the fractional part is 1.
// 
// If dn represents the nth digit of the fractional part, find the value of the following expression.
// 
// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
#![feature(test)]
extern crate test;

fn nth_champernowne_digit(n: u32) -> u32 {
    let mut count = 0;
    let mut digit = 0;
    while count <= n {
        digit += 1;
        count += (digit as f32).log10() as u32 + 1;
    }
    if n < 10 {
        return digit;
    }
    let position = count - n - 1;
    (digit / (10u32.pow(position))) % 10
}

pub fn solution() -> u32 {
    let mut product = 1;
    // We know that d1 and d10 are 1 so we skip them
    for i in 2..7 {
        product *= nth_champernowne_digit(10u32.pow(i) - 1);
    }
    product
}

fn main() {
    println!("The product is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(210, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
