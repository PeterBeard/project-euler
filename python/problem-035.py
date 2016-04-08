"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #035

The number, 197, is called a circular prime because all rotations of the digits:
197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71,
73, 79, and 97.

How many circular primes are there below one million?
"""
from math import log10, ceil

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


def rotate_digits(n):
    """Calculate the next rotation of the digits of n"""
    n_digits = ceil(log10(n + 1))
    # Pull off the last digit and stick it on the front
    return ((n % 10) * 10**(n_digits-1)) + (n // 10)


def is_circular_prime(n, is_prime):
    """Determine whether n is a circular prime."""
    if not is_prime[n]:
        return False
    rotated = rotate_digits(n)
    while rotated != n:
        if not is_prime[rotated]:
            return False
        rotated = rotate_digits(rotated)
    return True


def solution():
    sieve = prime_sieve(1000000)
    return len([n for n in range(2, 1000000) if is_circular_prime(n, sieve)])

print("There are {} circular primes below 1,000,000".format(solution()))
