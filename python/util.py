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
