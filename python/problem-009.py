"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
a^2 + b^2 = c^2

For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
"""
from math import sqrt, ceil, floor

def solution():
    for a in range(3, 1000):
        for b in range(a, 1000):
            c = sqrt(a**2 + b**2)
            if a + b + c == 1000 and ceil(c) == floor(c):
                return int(a*b*c)
    return 0

print("The product a*b*c where a + b + c = 1,000 is {}".format(solution()))
