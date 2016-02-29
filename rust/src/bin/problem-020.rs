// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// n! means n × (n − 1) × ... × 3 × 2 × 1
// 
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// 
// Find the sum of the digits in the number 100!
extern crate euler_util;
use euler_util::bigint::{BigInt, sum_of_digits};

fn main() {
    let value = BigInt::from_u32(100);

    println!("The sum of the digits of 100! is {}", sum_of_digits(&value.fact()));
}
