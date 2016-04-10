"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #037

The number 3797 has an interesting property. Being prime itself, it is possible
to continuously remove digits from left to right, and remain prime at each
stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797,
379, 37, and 3.

Find the sum of the only eleven primes that are both truncatable from left to
right and right to left.

NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
"""
from math import ceil, log10
from util import prime_sieve

def is_truncatable_prime(n, is_prime):
    """Determine whether n is a truncatable prime."""
    # Test LTR truncatability
    n_digits = ceil(log10(n + 1))
    for d in range(1, n_digits+1):
        if not is_prime[n % (10**d)]:
            return False

    # Test RTL truncatability
    while n >= 1:
        if not is_prime[n]:
            return False
        n //= 10

    return True

def solution():
    sieve = prime_sieve(1000000)
    return sum([n for n in range(10,1000000) if is_truncatable_prime(n, sieve)])

print("The sum of all left- and right-truncatable primes is {}".format(solution()))
