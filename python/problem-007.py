"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
the 6th prime is 13.

What is the 10 001st prime number?
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


def solution():
    count = 1
    n = 3
    while count < 10001:
        if is_prime(n):
            count += 1
        n += 2

    return n - 2

print("The 10,001st prime is {}".format(solution()))
