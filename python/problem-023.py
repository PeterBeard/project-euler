"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #23

A perfect number is a number for which the sum of its proper divisors is exactly
equal to the number. For example, the sum of the proper divisors of 28 would be
1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n
and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest
number that can be written as the sum of two abundant numbers is 24. By
mathematical analysis, it can be shown that all integers greater than 28123 can
be written as the sum of two abundant numbers. However, this upper limit cannot
be reduced any further by analysis even though it is known that the greatest
number that cannot be expressed as the sum of two abundant numbers is less than
this limit.

Find the sum of all the positive integers which cannot be written as the sum of
two abundant numbers.
"""

def d(n):
    """Find the sum of the proper divisors of n"""
    fsum = 1
    for f in range(2, n):
        if n % f == 0:
            fsum += f

    return fsum


def solution():
    abundants = []
    for i in range(12, 28123):
        if i < d(i):
            abundants.append(i)

    # Mark numbers in the list of integers if they can be written as the sum of
    # two abundant numbers
    impossibles = {}
    for n in range(0, 28123):
        impossibles[n] = 1

    for n in abundants:
        for m in abundants:
            s = m + n
            if s >= 28123:
                break
            if s in impossibles:
                impossibles[s] = 0

    return sum([k for k in impossibles if impossibles[k] == 1])

print("The sum of all positive integers that aren't the sum of two abundant numbers is {}".format(solution()))
