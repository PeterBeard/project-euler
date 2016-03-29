"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #10

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
"""
from util import is_prime

def solution():
    return sum([x for x in range(2,2000000) if is_prime(x)])

print("The sum of all primes < 2,000,000 is {}".format(solution()))
