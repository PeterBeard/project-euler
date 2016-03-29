"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #4

A palindromic number reads the same both ways. The largest palindrome made from
the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
"""
from math import log10

def num_digits(n):
    """Find the number of digits in n (minus 1)"""
    return int(log10(n))


def reverse_digits(n):
    """Reverse the digits of n"""
    if n < 10:
        return n
    else:
        return (n % 10) * 10**num_digits(n) + reverse_digits(int(n/10))


def solution():
    product = 0

    for x in range(100, 1000):
        for y in range(100, 1000):
            if x*y == reverse_digits(x*y):
                product = max(product, x*y)
    return product


print("The largest palindrome product is {}".format(solution()))
