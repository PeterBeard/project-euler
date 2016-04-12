"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #14

The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and finishing at 1)
contains 10 terms. Although it has not been proved yet (Collatz Problem), it
is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
"""

def collatz_length(n, memo=None):
    """
    Find the length of the Collatz sequence starting with n.
    
    Optional memo parameter is a dictionary of {n: collatz_length(n)}
    """
    length = 1
    while n > 1:
        if memo is not None and n in memo:
            return length+memo[n]-1

        if n % 2 == 0:
            n = n/2
        else:
            n = 3*n + 1
        length += 1

    return length

def solution():
    memo = {}
    max_length = 0
    max_n = 1
    for n in range(1, 1000000):
        length = collatz_length(n, memo)
        memo[n] = length
        if length > max_length:
            max_length = length
            max_n = n

    return max_n

print("The longest Collatz sequence starts with {}".format(solution()))
