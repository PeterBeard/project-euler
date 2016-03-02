// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Starting with the number 1 and moving to the right in a clockwise direction
// a 5 by 5 spiral is formed as follows:
// 
// 21 22 23 24 25
// 20  7  8  9 10
// 19  6  1  2 11
// 18  5  4  3 12
// 17 16 15 14 13
// 
// It can be verified that the sum of the numbers on the diagonals is 101.
// 
// What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?

fn main() {
    // An m by m grid will have (m - 1)/2 rings, each of which has (2n + 1)^2
    // in the top-left corner. The other corners are 2n, 4n, and 6n behind.
    // These are the diagonals
    let m = 1001;
    // Ring 0 only contains the number 1
    let mut sum = 1;
    for n in 1..((m-1)/2)+1 {
        let square = (2*n + 1)*(2*n + 1);
        sum += square;
        sum += square - 2*n;
        sum += square - 4*n;
        sum += square - 6*n;
    }
    println!("The sum of the diagonals of a {} by {} spiral is {}", m, m, sum);
}
