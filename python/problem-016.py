"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
"""

def sum_digits(n):
    """Calculate the sum of the digits of n"""
    s = 0
    while n >= 10:
        s += n % 10
        n //= 10
    s += n
    return s

def solution():
    # Again, arbitrary-precision to the rescue
    return sum_digits(2**1000)

print("The sum of the digits of 2^1000 is {}".format(solution()))
