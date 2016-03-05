// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Take the number 192 and multiply it by each of 1, 2, and 3:
// 
//     192 × 1 = 192
//     192 × 2 = 384
//     192 × 3 = 576
// 
// By concatenating each product we get the 1 to 9 pandigital, 192384576. We
// will call 192384576 the concatenated product of 192 and (1,2,3)
// 
// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4,
// and 5, giving the pandigital, 918273645, which is the concatenated product of
// 9 and (1,2,3,4,5).
// 
// What is the largest 1 to 9 pandigital 9-digit number that can be formed as
// the concatenated product of an integer with (1,2, ... , n) where n > 1?
//
#![feature(test)]
extern crate test;

/// Find a 9-digit number by multiplying n by (1, 2, ...)
fn nine_d_multiple(k: u32) -> u32 {
    let mut product = 0u64;
    let mut n = 1;
    //while (product as f64).log10() < 8f64 {
    while product < 100000000 {
        let digits = (n * k) as u64;
        let mut den = 1u64;
        while digits/den >= 1 {
            den *= 10;
        }
        product = product * den + digits;
        n += 1;
    }
    if product > 999999999 {
        return 0;
    }
    product as u32
}

/// Determine whether a number is pandigital
fn is_pandigital(n: u32) -> bool {
    let mut digits: [bool; 10] = [false; 10];
    for i in 0..9 {
        let d = (n % (10u32.pow(i+1)))/(10u32.pow(i));
        digits[d as usize] = true;
    }
    for i in 1..10 {
        if !digits[i] {
            return false;
        }
    }
    !digits[0]
}

pub fn solution() -> u32 {
    let mut max_multiple = 0;
    // The answer has to be < 10,000 since 10000 * [1, 2] has more than 9 digits
    for n in 2..10000 {
        let max = nine_d_multiple(n);
        if max > max_multiple && is_pandigital(max) {
            max_multiple = max;
        }
    }
    max_multiple
}

fn main() {
    println!("The largest pandigital 9-digit number formed as a concatenated product is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(932718654, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
