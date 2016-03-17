// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.
// ..
#![feature(test)]
extern crate test;

extern crate euler_util;
use euler_util::sum;

pub fn solution() -> u64 {
    // The sum should be on the order of 10^52, so we can lose the last 40
    // or so digits without affecting the first 10
    const VALUES: [u64; 100] = [
        3710728753,
        4637693767,
        7432498619,
        9194221336,
        2306758820,
        8926167069,
        2811287981,
        4427422891,
        4745144573,
        7038648610,
        6217645714,
        6490635246,
        9257586771,
        5820356532,
        8018119938,
        3539866437,
        8651550600,
        7169388870,
        5437007057,
        5328265410,
        3612327252,
        4587657617,
        1742370690,
        8114266041,
        5193432545,
        6246722164,
        1573244438,
        5503768752,
        1833638482,
        8038628759,
        7818283375,
        1672632010,
        4840309812,
        8708698755,
        5995940689,
        6979395067,
        4105268470,
        6537860736,
        3582903531,
        9495375976,
        8890280257,
        2526768027,
        3627021854,
        2407448690,
        9143028819,
        3441306557,
        2305308117,
        1148769693,
        6378329949,
        6772018697,
        9554825530,
        7608532713,
        3777424253,
        2370191327,
        2979886027,
        1849570145,
        3829820378,
        3482954382,
        4095795306,
        2974615218,
        4169811622,
        6246795719,
        2318970677,
        8618808822,
        1130673970,
        8295917476,
        9762333104,
        4284628018,
        5512160354,
        3223819573,
        7550616496,
        6217784275,
        3292418570,
        9951867143,
        7326746080,
        7684182252,
        9714261791,
        8778364618,
        1084880252,
        7132961247,
        6218407357,
        6662789198,
        6066182629,
        8578694408,
        6602439640,
        6491398268,
        1673093931,
        9480937724,
        7863916702,
        1536871371,
        4078992311,
        4488991150,
        4150312888,
        8123488067,
        8261657077,
        2291880205,
        7715854250,
        7210783843,
        2084960398,
        5350353422
    ];

    let mut sum = sum::<u64>(&VALUES[..], 0);

    // Truncate to 10 digits
    while sum > 10f64.powi(10) as u64 {
        sum = (sum as f64 / 10f64).round() as u64;
    }
    sum
}

fn main() {
    println!("The first 10 digits of the sum are {}", solution());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn correct() {
        assert_eq!(5537376230, solution());
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        b.iter(|| solution());
    }
}
