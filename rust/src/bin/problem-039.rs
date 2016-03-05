// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// If p is the perimeter of a right angle triangle with integral length sides,
// {a,b,c}, there are exactly three solutions for p = 120.
// 
// {20,48,52}, {24,45,51}, {30,40,50}
// 
// For which value of p ≤ 1000, is the number of solutions maximised?
//
#![feature(test)]
extern crate test;

/// Find the number of right triangles with integral sides for a given perimeter
fn num_triangles(p: u32) -> u32 {
    let mut count = 0;
    let psq = p*p;
    // a^2 + b^2 = c^2, a + b + c = p; substitute c = p - a - b and rearrange
    // solutions must satisfy 2p(a+b) = p^2 + ab
    // TODO: This can probably be simplified further, but algebra
    for a in 1..p {
        for b in 1..p {
            if 2*p*(a + b) == psq + a*b {
                count += 1;
            }
        }
    }
    count
}

pub fn solution() -> u32 {
    let mut max = 0;
    let mut max_p = 0;
    for p in 1..1001 {
        // Perimeter will always be even
        if p % 2 != 0 {
            continue;
        }
        let count = num_triangles(p);
        if count > max {
            max = count;
            max_p = p;
        }
    }
    max_p
}

fn main() {
    println!("p = {} has the maximum number of solutions.", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(840, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
