// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// You are given the following information, but you may prefer to do some research for yourself.
// 
//     1 Jan 1900 was a Monday.
//     Thirty days has September,
//     April, June and November.
//     All the rest have thirty-one,
//     Saving February alone,
//     Which has twenty-eight, rain or shine.
//     And on leap years, twenty-nine.
//     A leap year occurs on any year evenly divisible by 4, but not on a
//         century unless it is divisible by 400.
// 
// How many Sundays fell on the first of the month during the twentieth century
// (1 Jan 1901 to 31 Dec 2000)?
#![feature(test)]
extern crate test;

use std::fmt;

/// The days of the week
#[derive(Debug, PartialEq)]
enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

/// Date described by year, month, and day
struct Date {
    day: u32,
    month: u32,
    year: u32,
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

/// Get the day of the week for a particular date
///
/// Formula from https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week
fn get_day_of_week(day: Date) -> Day {
    let m = match day.month {
        1 => 0,
        2 => 3,
        3 => 2,
        4 => 5,
        5 => 0,
        6 => 3,
        7 => 5,
        8 => 1,
        9 => 4,
        10 => 6,
        11 => 2,
        _ => 4,
    };
    let y = match day.month {
        1 | 2 => (day.year - 1) % 100,
        _     => day.year % 100,
    };
    let y = (y + y/4) % 7;
    let c = match (day.year/100) % 4 {
        0 => 0,
        1 => 5,
        2 => 3,
        _ => 1,
    };
    match (day.day + m + y + c) % 7{
        0 => Day::Sunday,
        1 => Day::Monday,
        2 => Day::Tuesday,
        3 => Day::Wednesday,
        4 => Day::Thursday,
        5 => Day::Friday,
        _ => Day::Saturday,
    }
}

pub fn solution() -> u32 {
    let start_date = Date { day: 1, month: 1, year: 1901 };
    let end_date = Date { day: 31, month: 12, year: 2000 };
    let mut count = 0;
    // Count all of the Sundays that are on the first of the month
    for year in start_date.year..end_date.year + 1 {
        for month in 1..end_date.month + 1 {
            if get_day_of_week(Date{ day: 1, month: month, year: year }) == Day::Sunday {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    println!("{} Sundays occur on the first of the month between Jan 1, 1901 and Dec 31, 2000", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(171, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
