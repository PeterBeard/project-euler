// Copyright 2016 Peter Beard
// Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
//
// Library for doing arithmetic with large integers

/// A big integer. Digits are stored little-endian, i.e. the least significant digit is at
/// digits[0]. BigInts are unsigned and have no upper bound other than available memory.
#[derive(Debug, Clone, PartialEq)]
pub struct BigInt {
    digits: Vec<u8>
}

impl BigInt {
    /// Make a new BigInt
    pub fn new() -> BigInt {
        BigInt {
            digits: Vec::new(),
        }
    }

    /// Make a new BigInt from a u32
    pub fn from_u32(value: u32) -> BigInt {
        let mut bigint = BigInt::new();
        // How many digits will we need
        let digit_count = ((value as f32).log10() + 1f32) as u32;

        for d in 0..digit_count {
            let digit = (value / 10u32.pow(d)) % 10;
            bigint.digits.push(digit as u8);
        }
        if digit_count == 0 {
            bigint.digits.push(0);
        }
        bigint
    }

    /// Make a new BigInt from a u64
    pub fn from_u64(value: u64) -> BigInt {
        let mut bigint = BigInt::new();
        // How many digits will we need
        let digit_count = ((value as f64).log10() + 1f64) as u32;

        for d in 0..digit_count {
            let digit = (value / 10u64.pow(d)) % 10;
            bigint.digits.push(digit as u8);
        }
        if digit_count == 0 {
            bigint.digits.push(0);
        }
        bigint
    }

    /// Remove leading zeros
    pub fn strip_zeros(&mut self) {
        while self.digits.len() > 1 && self.digits[self.digits.len() - 1] == 0 {
            self.digits.pop();
        }
    }

    /// Add two BigInts
    pub fn add(&self, other: &BigInt) -> BigInt {
        // We want the argument with fewer digits first
        if other.digits.len() < self.digits.len() {
            return other.add(&self);
        }
        
        let mut sum = BigInt::new();
        let mut carry = false;
        for i in 0..self.digits.len() {
            let mut val = self.digits[i] + other.digits[i];
            if carry {
                val += 1;
            }
            sum.digits.push(val % 10);
            carry = val > 9;
        }
        // Add any remaining digits from the other value
        if other.digits.len() > self.digits.len() {
            for i in self.digits.len()..other.digits.len() {
                if carry {
                    carry = other.digits[i] > 8;
                    sum.digits.push((other.digits[i] + 1) % 10);
                } else {
                    sum.digits.push(other.digits[i]);
                }
            }
        }
        if carry {
            sum.digits.push(1);
        }
        sum
    }

    /// Subtract two BigInts
    ///
    /// Note that you will get weird results if the answer would be negative
    pub fn sub(&self, other: &BigInt) -> BigInt {
        // Get the 10s complement of the other value
        let mut complement = BigInt::new();
        for i in 0..self.digits.len() {
            if i < other.digits.len() {
                complement.digits.push(9 - other.digits[i]);
            } else {
                complement.digits.push(9);
            }
        }
        complement = complement.add(&BigInt::from_u32(1));

        // Add the 10s complement and discard the leftmost digit
        let mut diff = self.add(&complement);

        diff.digits.pop();
        diff.strip_zeros();
        diff
    }

    /// Multiply two BigInts
    pub fn mul(&self, other: &BigInt) -> BigInt {
        let mut product = BigInt::new();
        let mut shift = 0;

        for sd in &self.digits {
            let mut partial_product = BigInt::new();
            for _ in 0..shift {
                partial_product.digits.push(0);
            }
            shift += 1;

            let mut carry = 0;
            for od in &other.digits {
                let p = od * sd;
                let digit = p % 10 + carry;
                partial_product.digits.push(digit % 10);

                carry = 0;
                if p > 0 {
                    carry = p/10;
                }
                if digit > 0 {
                    carry += digit/10;
                }
            }
            if carry > 0 {
                partial_product.digits.push(carry % 10);
            }
            product = product.add(&partial_product);
        }
        product.strip_zeros();
        product
    }

    /// Calculate n! for a BigInt
    pub fn fact(self) -> BigInt {
        let one = BigInt::from_u32(1);
        let mut value = BigInt::from_u32(1);
        let mut product = BigInt::from_u32(1);
        while value != self {
            value = value.add(&one);
            product = product.mul(&value);
        }
        product
    }
}

/// Add the digits in a BigInt
pub fn sum_of_digits(n: &BigInt) -> u32 {
    let mut sum = 0;
    for &d in &n.digits {
        sum += d as u32;
    }
    sum
}

#[test]
fn test_add() {
    let a = BigInt::from_u32(999);
    let b = BigInt::from_u32(1);
    let c = BigInt::from_u32(1000);

    assert_eq!(c, a.add(&b));
}

#[test]
fn test_sub() {
    let a = BigInt::from_u32(727);
    let b = BigInt::from_u32(28);
    let c = BigInt::from_u32(699);

    assert_eq!(c, a.sub(&b));
}

#[test]
fn test_mul() {
    let a = BigInt::from_u32(99);
    let b = BigInt::from_u32(99);
    let c = BigInt::from_u32(9801);

    assert_eq!(c, a.mul(&b));
}

#[test]
fn test_fact() {
    // This is the biggest factorial that fits in a u64
    let a = BigInt::from_u32(20);
    let b = BigInt::from_u64(2432902008176640000);

    assert_eq!(b, a.fact());
}
