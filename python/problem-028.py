"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #028

Starting with the number 1 and moving to the right in a clockwise direction a 5
by 5 spiral is formed as follows:

21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed
in the same way?
"""
def ring_sum(n):
    """
    Find the sum of the corners of ring n in a spiral
    ring 0 is the center (1), ring 1 is the next ring (2-9), etc.
    """
    if n <= 0:
        return 1
    # The value in the upper right corner is related to the square of the ring
    # index. The other corners can be calculated from this value.
    n_sq = (2*n+1)*(2*n+1)
    return n_sq + (n_sq - 2*n) + (n_sq - 4*n) + (n_sq - 6*n)


def solution():
    return sum([ring_sum(n) for n in range(0,501)])

print("The sum of the diagonals in a 1001x1001 spiral is {}".format(solution()))
