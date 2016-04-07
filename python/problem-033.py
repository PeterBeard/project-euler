"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #033

The fraction 49/98 is a curious fraction, as an inexperienced mathematician in
attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is
correct, is obtained by cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial examples.

There are exactly four non-trivial examples of this type of fraction, less than
one in value, and containing two digits in the numerator and denominator.

If the product of these four fractions is given in its lowest common terms, find
the value of the denominator.
"""
from fractions import gcd

def solution():
    fracs = []
    for n_tens in range(1, 10):
        for n_ones in range(1, 10):
            for d_tens in range(1, 10):
                for d_ones in range(1, 10):
                    num = n_tens * 10 + n_ones
                    den = d_tens * 10 + d_ones
                    f = num/den
                    if f >= 1:
                        break
                    # Cancel the ones of the numerator and the tens of the
                    # denominator to see if the fraction can be "reduced"
                    if n_ones == d_tens and n_tens/d_ones == f:
                        fracs.append((num, den))
    product = (1, 1)
    for f in fracs:
        product = (product[0]*f[0], product[1]*f[1])

    return product[1] // gcd(product[0], product[1])
    

print("The denominator of the product of all curious fractions is {}".format(solution()))
