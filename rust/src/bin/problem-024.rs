// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
// 
// A permutation is an ordered arrangement of objects. For example, 3124 is one
// possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
// are listed numerically or alphabetically, we call it lexicographic order.
// The lexicographic permutations of 0, 1 and 2 are:
// 
// 012   021   102   120   201   210
// 
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4,
// 5, 6, 7, 8 and 9?
#![feature(test)]
extern crate test;

/// Find the next lexicographic permutation of a set of digits
fn next_permutation(digits: &[u32]) -> Vec<u32> {
    let mut next: Vec<u32> = digits.to_vec();
    // First we need to find the start of the largest non-increasing suffix
    let mut pivot = 0;
    let mut swap = 0;
    for i in 1..digits.len() {
        if digits[i] > digits[i-1] {
            pivot = i-1;
        }
        if digits[i] > digits[pivot] {
            swap = i;
        }
    }
    // Now we swap the pivot and the rightmost greater value
    next[pivot] = digits[swap];
    next[swap] = digits[pivot];

    // Now we sort the suffix and we're done
    let mut suffix: Vec<u32> = next.split_off(pivot+1);
    suffix.sort();
    next.append(&mut suffix);
    next
}

pub fn solution() -> Vec<u32> {
    // Permute the digits 0-9 a million times
    let mut digits: Vec<u32> = vec![0,1,2,3,4,5,6,7,8,9];
    for _ in 1..1000000 {
        digits = next_permutation(&digits[..]);
    }
    digits
}

fn main() {
    let digits = solution();
    print!("The 1,000,000th permutation is ");
    for d in &digits {
        print!("{}", d);
    }
    print!("\n");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(vec![2,7,8,3,9,1,5,4,6,0], solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
