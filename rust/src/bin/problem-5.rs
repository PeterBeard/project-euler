// 2520 is the smallest number that can be divided by each of the numbers
// from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of
// the numbers from 1 to 20?

fn main() {
    let mut n = 0;
    let mut all_divisible = false;
    // This might take a while; 20! is pretty big...
    while !all_divisible {
        n += 1;
        all_divisible = true;
        for f in 2..21 {
            if n % f != 0 {
                all_divisible = false;
                break;
            }
        }
    }
    println!("The smallest positive integer divisible by all of [1,20] is {}", n);
}
