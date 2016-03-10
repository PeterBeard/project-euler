// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
#![feature(test)]
extern crate test;

/// Compute n^p mod b
fn modular_pow(n: u64, p: u64, b: u64) -> u64 {
    let mut pow = n % b;
    for _ in 1..p {
        pow = (pow * (n % b)) % b;
    }
    pow
}

pub fn solution() -> u64 {
    const BASE: u64 = 10000000000;
    let mut sum = 0;
    for n in 1..1001 {
        sum += modular_pow(n, n, BASE);
    }
    sum % BASE
}

fn main() {
    println!("The sum is {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(9110846700, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
