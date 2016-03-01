// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
// 
// The Fibonacci sequence is defined by the recurrence relation:
// 
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// 
// Hence the first 12 terms will be:
// 
//     F1 = 1
//     F2 = 1
//     F3 = 2
//     F4 = 3
//     F5 = 5
//     F6 = 8
//     F7 = 13
//     F8 = 21
//     F9 = 34
//     F10 = 55
//     F11 = 89
//     F12 = 144
// 
// The 12th term, F12, is the first term to contain three digits.
// 
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
extern crate euler_util;
use euler_util::bigint::BigInt;

fn main() {
    let mut a = BigInt::from_u32(1);
    let mut b = BigInt::from_u32(1);
    let mut c = BigInt::new();
    let mut index = 2;

    while c.num_digits() < 1000 {
        c = a.add(&b);
        a = b.clone();
        b = c.clone();
        index += 1;
    }

    println!("The first Fibonacci number with 1000 digits is number {}.", index);
}
