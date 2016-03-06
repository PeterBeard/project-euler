// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten
// triangle numbers are:
// 
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// 
// By converting each letter in a word to a number corresponding to its alphabetical position and
// adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 =
// 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
// 
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
// two-thousand common English words, how many are triangle words?
#![feature(test)]
extern crate test;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Calculate the value of a word
fn value(word: &str) -> u32 {
    let mut v = 0;
    for c in word.chars() {
        if c.is_alphabetic() {
            v += c as u32 - 64;
        }
    }
    v
}

/// Determine whether a number is a triangle number
fn is_triangle_number(t: u32) -> bool {
    let mut n = (t as f32).sqrt() as u32;
    let mut tri = 0;
    while tri < t {
        tri = n*(n+1)/2;
        n += 1;
    }
    tri == t
}

pub fn solution(data_file: &str) -> u32 {
    // Load the words from the file
    let f = match File::open(data_file) {
        Ok(file) => file,
        Err(e) => panic!("Failed to read file: {}", e)
    };
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(len) => len,
        Err(e) => panic!("Failed to read line: {}", e)
    };

    let words: Vec<&str> = line.split(',').collect();

    let mut count = 0;
    for w in words {
        if is_triangle_number(value(w)) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("Found {} triangle words.",
             solution("../../../data/p042_words.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(162, solution("../data/p042_words.txt"));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution("../data/p042_words.txt"));
    }
}
