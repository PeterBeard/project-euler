"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #15

Starting in the top left corner of a 2×2 grid, and only being able to move to
the right and down, there are exactly 6 routes to the bottom right corner.

How many such routes are there through a 20×20 grid?
"""

def binomial_c(n, k):
    """Find the binomial coefficient (n k)"""
    n += 1
    coeff = (n - k) / k
    for i in range(1, k):
        coeff *= (n - i) / i
    return round(coeff)

def solution():
    # The number of routes from (0, 0) to (n, k) is just (n + k n)
    # So, the number of routes to (20, 20) is:
    return binomial_c(40, 20)

print("The number of routes through a 20x20 grid is {}".format(solution()))
