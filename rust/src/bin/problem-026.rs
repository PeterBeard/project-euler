// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
// 
// A unit fraction contains 1 in the numerator. The decimal representation of
// the unit fractions with denominators 2 to 10 are given:
// 
//     1/2	= 	0.5
//     1/3	= 	0.(3)
//     1/4	= 	0.25
//     1/5	= 	0.2
//     1/6	= 	0.1(6)
//     1/7	= 	0.(142857)
//     1/8	= 	0.125
//     1/9	= 	0.(1)
//     1/10	= 	0.1 
// 
// Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be
// seen that 1/7 has a 6-digit recurring cycle.
// 
// Find the value of d < 1000 for which 1/d contains the longest recurring
// cycle in its decimal fraction part.

/// Find the length of the repetend of 1/d in decimal
fn repeating_length(d: u32) -> u32 {
    let mut remainders: Vec<u32> = Vec::new();
    let mut n = 1;
    
    loop {
        if n < d {
            n *= 10;
        }
        let rem = n % d;
        if remainders.contains(&rem) {
            // Find the location of the first instance of this remainder
            let mut count = 0;
            for r in &remainders {
                if r == &rem {
                    break;
                }
                count += 1;
            }
            return (remainders.len() - count) as u32;
        } else {
            remainders.push(rem);
            n = rem;
        }
    }
}

fn main() {
    let mut max = 0;
    let mut max_d = 0;
    for d in 1..1000 {
        let r = repeating_length(d);
        if r > max {
            max = r;
            max_d = d;
        }
    }
    println!("{} has the longest repetend for d < 1000.", max_d);
}
