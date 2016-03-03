// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to
// simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by
// cancelling the 9s.
// 
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
// 
// There are exactly four non-trivial examples of this type of fraction, less than one in value,
// and containing two digits in the numerator and denominator.
// 
// If the product of these four fractions is given in its lowest common terms, find the value of
// the denominator.

/// Reduce a fraction to lowest common terms
fn reduce_fraction(n: u32, d: u32) -> (u32, u32) {
    let max_factor = if n > d { n } else { d };
    let mut factor = max_factor;
    let mut r_n = n;
    let mut r_d = d;

    while factor > 1 {
        if r_n % factor == 0 && r_d % factor == 0 {
            r_n /= factor;
            r_d /= factor;
            factor = max_factor / factor;
        } else {
            factor -= 1;
        }
    }
    (r_n, r_d)
}

fn main() {
    let mut n_product = 1f32;
    let mut d_product = 1f32;
    for numerator in 1..10 {
        for denominator in (numerator+1)..10 {
            // Try the digits 1-9 as the cancelling digit
            let value:f32 = numerator as f32 / denominator as f32;
            for digit in 1..10 {
                let num_1 = (digit*10 + numerator) as f32;
                let num_2 = (digit + numerator*10) as f32;
                let den_1 = (digit*10 + denominator) as f32;
                let den_2 = (digit + denominator*10) as f32;
                
                if num_1/den_1 == value {
                    n_product *= num_1;
                    d_product *= den_1;
                } else if num_1/den_2 == value {
                    n_product *= num_1;
                    d_product *= den_2;
                } else if num_2/den_1 == value {
                    n_product *= num_2;
                    d_product *= den_1;
                } else if num_2/den_2 == value {
                    n_product *= num_2;
                    d_product *= den_2;
                }
            }
        }
    }
    let (_, d) = reduce_fraction(n_product as u32, d_product as u32);
    println!("The denominator of the product of the fractions is {}", d);
}
