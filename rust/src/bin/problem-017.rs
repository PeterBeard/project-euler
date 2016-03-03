// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// If the numbers 1 to 5 are written out in words: one, two, three, four, five,
// then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
// 
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out
// in words, how many letters would be used?
// 
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
// forty-two) contains 23 letters and 115 (one hundred and fifteen) contains
// 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
#![feature(test)]
extern crate test;

/// Get the English word for a particular number in [1, 1000]
fn number_as_word(n: u32) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        21...99 => number_as_word((n/10)*10) + &String::from("-") + &(number_as_word(n%10)),
        100 | 200 | 300 | 400 | 500 | 600 | 700 | 800 | 900 => 
            number_as_word(n/100) + &(String::from(" hundred")),
        101...999 => number_as_word((n/100)*100) + &(String::from(" and ")) + &(number_as_word(n%100)),
        1000 => String::from("one thousand"),
        _ => String::from("eleventeen"),
    }
}

pub fn solution() -> u32 {
    let mut char_count = 0;
    for n in 1..1001 {
        // Get the word and strip out hyphens and spaces
        char_count += number_as_word(n).replace("-", "").replace(" ", "").len();
    }
    char_count as u32
}

fn main() {
    println!("Written out, the numbers from 1 to 1000 have {} non-whitespace characters.", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(21124, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
