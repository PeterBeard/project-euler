"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #026

A unit fraction contains 1 in the numerator. The decimal representation of the
unit fractions with denominators 2 to 10 are given:

    1/2	= 	0.5
    1/3	= 	0.(3)
    1/4	= 	0.25
    1/5	= 	0.2
    1/6	= 	0.1(6)
    1/7	= 	0.(142857)
    1/8	= 	0.125
    1/9	= 	0.(1)
    1/10 =      0.1 

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be
seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in
its decimal fraction part.
"""

def repeating_cycle(num, denom):
    """
    Find the length of the repeating part of the decimal expansion of the fraction
    (num/denom)
    """
    remainders = []
    rem = 0
    n = num

    # Calculate the decimal expansion of n/d
    while True:
        if n < denom:
            n *= 10
        rem = n % denom
        if rem in remainders:
            break
        remainders.append(rem)
        n = rem

    # Find the beginning of the repeating part
    for (i, r) in enumerate(remainders):
        if r == rem:
            return len(remainders) - i
    return 0

def solution():
    return max(
        [(i, repeating_cycle(1, i)) for i in range(2,1000)],
        key=lambda t: t[1],
    )[0]

print("The denominator below 1,000 with the longest repetend is {}".format(solution()))
