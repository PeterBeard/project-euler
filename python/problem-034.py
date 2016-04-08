"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #034

145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the factorial of their
digits.

Note: as 1! = 1 and 2! = 2 are not sums they are not included.
"""
from util import digits

def fact(n):
    """Calculate n!"""
    if n <= 1:
        return 1
    return n * fact(n - 1)

def solution():
    return sum([n for n in range(3, 50000) if (sum(map(fact, digits(n))) == n)])

print("The sum of all numbers where the sum of the factorials of the digits equals the number is {}".format(solution()))
