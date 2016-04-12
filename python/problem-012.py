"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #12

The sequence of triangle numbers is generated by adding the natural numbers. So
the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten
terms would be:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:

     1: 1
     3: 1,3
     6: 1,2,3,6
    10: 1,2,5,10
    15: 1,3,5,15
    21: 1,3,7,21
    28: 1,2,4,7,14,28

We can see that 28 is the first triangle number to have over five divisors.

What is the value of the first triangle number to have over five hundred divisors?
"""
from math import sqrt

def num_factors(n):
    """Find the number of factors of n"""
    max_factor = int(sqrt(n) + 1)
    count = 2
    for i in range(2, max_factor):
        if n % i == 0:
            # Each factor has a partner > sqrt(n)
            count += 2
    return count


def solution():
    value = 28
    n = 8
    while num_factors(value) < 500:
        value += n
        n += 1
    return value


print("The first triangle number with more than 500 divisors is {}".format(solution()))
