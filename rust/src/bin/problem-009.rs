// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2
// 
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// 
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
#![feature(test)]
extern crate test;

pub fn solution() -> u32 {
    for a in 1..1000 {
        let asq = a*a;
        for b in a..1000 {
            let bsq = b*b;
            for c in b..1000 {
                let csq = c*c;
                if a+b+c == 1000 && (asq + bsq) == csq {
                    return a*b*c;
                }
            }
        }
    }
    0
}

fn main() {
    println!("The product of a, b, and c where a + b + c = 1000 is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(31875000, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
