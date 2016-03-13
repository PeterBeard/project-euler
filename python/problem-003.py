"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
"""
from math import sqrt

def prime_factorization(n):
    """List the prime factors of n"""
    factors = []
    max_factor = int(sqrt(n)) + 1
    for f in range(2,max_factor):
        while n % f == 0:
            factors.append(f)
            n /= f
    if len(factors) == 0:
        return [n]
    else:
        return factors

def solution():
    return max(prime_factorization(600851475143))

print("The largest prime factor of 600,851,475,143 is {}".format(solution()))
