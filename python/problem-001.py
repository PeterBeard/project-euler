"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #1

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
"""

def solution():
    return sum(filter(lambda x: x % 3 == 0 or x % 5 == 0, range(1, 1000)))

print("The sum of the multiples of 3 or 5 under 1,000 is {}".format(solution()))
