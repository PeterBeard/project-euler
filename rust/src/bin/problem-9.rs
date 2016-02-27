// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2
// 
// For example, 32 + 42 = 9 + 16 = 25 = 52.
// 
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn main() {
    'main: for a in 1..1000 {
        let asq = a*a;
        for b in a..1000 {
            let bsq = b*b;
            for c in b..1000 {
                let csq = c*c;
                if a+b+c == 1000 && (asq + bsq) == csq {
                    println!("{}^2 + {}^2 = {}^2, {}*{}*{} = {}", a, b, c, a, b, c, a*b*c);
                    break 'main;
                }
            }
        }
    }
}
