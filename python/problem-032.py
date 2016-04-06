"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #032

We shall say that an n-digit number is pandigital if it makes use of all the
digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through
5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing
multiplicand, multiplier, and product is 1 through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product identity can
be written as a 1 through 9 pandigital.
HINT: Some products can be obtained in more than one way so be sure to only
include it once in your sum.
"""

def digits(n):
    """Return a list of the digits of n"""
    digits = []
    while n > 0:
        digits.insert(0, n%10)
        n //= 10
    return digits


def is_pandigital_product(a, b):
    """Determine whether a * b is a pandigital product."""
    missing = [True]*9
    pandigits = [1, 2, 3, 4, 5, 6, 7, 8, 9]

    if a*b < 10**8:
        total_digits = digits(a) + digits(b) + digits(a*b)

        if len(total_digits) != 9:
            return False
        for d in total_digits:
            if d == 0:
                return False
            missing[d-1] = False

    return not any(missing)


def solution():
    products = []

    for a in range(2, 5000):
        for b in range(2, a):
            if is_pandigital_product(a, b):
                products.append(a*b)

    return sum(set(products))


print("The sum of all pandigital products is {}".format(solution()))
