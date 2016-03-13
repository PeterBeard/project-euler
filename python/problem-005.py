"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

2520 is the smallest number that can be divided by each of the numbers from 1 to
10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the
numbers from 1 to 20?
"""
def is_divisible(n, factors):
    """Determine whether n is divisible by all of [factors]"""
    for f in factors:
        if n % f != 0:
            return False
    return True


def solution():
    n = 2520
    while True:
        n += 20
        if is_divisible(n, range(20, 10, -1)):
            return n

print("The smallest number divisible by all of [1,20] is {}".format(solution()))
