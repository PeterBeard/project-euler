"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Various frequently-used functions
"""
from math import sqrt

def is_prime(n):
    """Determine whether a number is prime"""
    if n < 2:
        return False

    max_factor = int(sqrt(n)) + 1
    for f in range(2, max_factor):
        if n % f == 0:
            return False

    return True


def prime_sieve(upper_bound):
    """Mark all integers from 1 to upper_bound as prime (True) or composite (False)."""
    primes = [True] * upper_bound
    primes[0] = False
    primes[1] = False

    for n in range(2, upper_bound):
        # Mark every multiple of n as composite
        if primes[n]:
            c = 2*n
            while c < upper_bound:
                primes[c] = False
                c += n
    return primes


def digits(n):
    """Return a list of the digits of n"""
    digits = []
    while n > 0:
        digits.insert(0, n%10)
        n //= 10
    return digits
