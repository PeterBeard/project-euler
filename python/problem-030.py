"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #030

Surprisingly there are only three numbers that can be written as the sum of
fourth powers of their digits:

    1634 = 1^4 + 6^4 + 3^4 + 4^4
    8208 = 8^4 + 2^4 + 0^4 + 8^4
    9474 = 9^4 + 4^4 + 7^4 + 4^4

As 1 = 1^4 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers
of their digits.
"""
from util import digits

def fifth_digits(n):
    """Return the sum of the digits of n raised to the fifth power"""
    return sum(map(lambda x: x**5, digits(n)))

def solution():
    return sum([x for x in range(2, 531442) if x == fifth_digits(x)])

print("The sum of all numbers that can be written as the sum of the fifth powers of their digits is {}".format(solution()))
