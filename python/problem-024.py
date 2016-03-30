"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #024

A permutation is an ordered arrangement of objects. For example, 3124 is one
possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are
listed numerically or alphabetically, we call it lexicographic order. The
lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5,
6, 7, 8 and 9?
"""

def permute_digits(s):
    """Return the next lexicographic permutation of the given list of digits."""
    substr_start = 0    # Start of the non-ascending substring
    pivot = 0           # Position of the next largest digit in the substring

    for i in range(len(s)-1, 0, -1):
        c = s[i]
        next_c = s[i-1]
        if next_c < c:
            substr_start = i
            break

    for i in range(len(s)-1, substr_start-1, -1):
        if s[i] > s[substr_start-1]:
            pivot = i
            break

    new_s = s[::1]
    new_s[substr_start-1] = s[pivot]
    new_s[pivot] = s[substr_start-1]
    return new_s[:substr_start] + sorted(new_s[substr_start:])
    

def solution():
    digits = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]

    for i in range(1,1000000):
        digits = permute_digits(digits)

    return ''.join([str(d) for d in digits])

print("The millionth permutation of the digits 0-9 is {}".format(solution()))
