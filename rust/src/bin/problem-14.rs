// The following iterative sequence is defined for the set of positive integers:
// 
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
// 
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// 
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
// 
// Which starting number, under one million, produces the longest chain?
// 
// NOTE: Once the chain starts the terms are allowed to go above one million.

/// Get the length of the Collatz sequence for n
fn collatz_length(n: u64) -> u64 {
    let mut k = n;
    let mut length = 1;
    if k < 2 {
        return 1;
    }
    while k != 1 {
        if k % 2 == 0 {
            k /= 2;
        } else {
            k = 3*k + 1;
        }
        length += 1;
    }
    length
}

fn main() {
    let mut max_length = 0;
    let mut max_length_n = 0;

    for n in 16..1000000 {
        let l = collatz_length(n);
        if l > max_length {
            max_length = l;
            max_length_n = n;
        }
    }
    println!("Longest started with {} (length {})", max_length_n, max_length)
}
