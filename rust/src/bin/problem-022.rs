// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Using names.txt (right click and 'Save Link/Target As...'), a 46K text
// file containing over five-thousand first names, begin by sorting it into
// alphabetical order. Then working out the alphabetical value for each name,
// multiply this value by its alphabetical position in the list to obtain a name score.
// 
// For example, when the list is sorted into alphabetical order, COLIN, which
// is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN
// would obtain a score of 938 × 53 = 49714.
// 
// What is the total of all the name scores in the file?
#![feature(test)]
extern crate test;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Calculate the score for a name
fn score(name: &str) -> u32 {
    let mut s = 0;
    for c in name.chars() {
        if c.is_alphabetic() {
            s += c as u32 - 64;
        }
    }
    s
}

pub fn solution(data_file: &str) -> u32 {
    let f = match File::open(data_file) {
        Ok(file) => file,
        Err(e) => panic!("Failed to read file: {}", e)
    };
    let mut reader = BufReader::new(f);

    // We're only interested in the first line
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(len) => len,
        Err(e) => panic!("Failed to read line: {}", e)
    };

    let mut names: Vec<&str> = line.split(',').collect();
    names.sort();
    let scores: Vec<u32> = names.into_iter().map(score).collect();
    let mut sum = 0;
    for i in 0..scores.len() {
        sum += scores[i] * (i as u32 + 1);
    }
    sum
}

fn main() {
    println!("The sum of the scores of the alphabetized names is {}",
             solution("../../../data/p022_names.txt")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(871198282, solution("../data/p022_names.txt"));
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution("../data/p022_names.txt"));
    }
}
