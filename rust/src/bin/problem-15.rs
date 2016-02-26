// Starting in the top left corner of a 2×2 grid, and only being able to move to
// the right and down, there are exactly 6 routes to the bottom right corner.
//
// How many such routes are there through a 20×20 grid?
use std::fmt;

/// Unsigned 2-d point
struct Point {
    x: u64,
    y: u64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Calculate the binary coefficient (n k)
///
/// This algorithm uses a product rather than a pair of potentially huge factorials
/// \Pi_{i=1}^{k}\frac{n+1-i}{i}
fn binomial_coefficient(n: u64, k: u64) -> u64 {
    // Need to use floats to allow for accurate division
    let nplus1 = n as f64 + 1.0;
    let mut coeff:f64 = (nplus1 - k as f64) / k as f64;
    for i in 1..k {
        coeff *= (nplus1 - i as f64)/i as f64;
    }
    coeff.round() as u64
}

/// How many routes are there from where we are to the goal?
///
/// The number of lattice paths from (0, 0) to (n, k) is
/// the binomial coefficient (n + k n)
fn routes_to_point(p: &Point) -> u64 {
    println!("Calculting {} C {}", p.x+p.y, p.y);
    binomial_coefficient((p.x + p.y), p.x)
}

fn main() {
    let goal = Point { x: 20, y: 20 };

    println!("There are {} routes from (0, 0) to {}", routes_to_point(&goal), goal);
}
