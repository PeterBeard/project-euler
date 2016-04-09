"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #036

The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.

Find the sum of all numbers, less than one million, which are palindromic in
base 10 and base 2.

(Please note that the palindromic number, in either base, may not include
leading zeros.)

"""
from util import digits

def is_palindrome(n):
    """Determine whether n is a palindrome."""
    d = digits(n)
    return d == d[::-1]


def is_b_palindrome(n):
    """Determine whether n is a palindrome in base 2."""
    fw = "{:b}".format(n)
    return fw == fw[::-1]

def solution():
    return sum([n for n in range(1, 1000000) if is_palindrome(n) and is_b_palindrome(n)])

print("The sum of all base-10 and base-2 palindromes less than 1,000,000 is {}".format(solution()))
