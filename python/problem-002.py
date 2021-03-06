"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #2

Each new term in the Fibonacci sequence is generated by adding the previous two
terms. By starting with 1 and 2, the first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values do not exceed
four million, find the sum of the even-valued terms.
"""

def fibs(n):
    """List the Fibonacci numbers less than n"""
    f = [1, 2]
    v = 3
    while v < n:
        f.append(v)
        v = f[-1] + f[-2]
    return f


def solution():
    return sum([x for x in fibs(4000000) if x % 2 == 0])


print("The sum of the even Fibonacci numbers under 4,000,000 is {}".format(solution()))
