"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Let d(n) be defined as the sum of proper divisors of n (numbers less than n
which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and
each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55
and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and
142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
"""
def d(n):
    """Find the sum of the proper divisors of n"""
    fsum = 1
    for f in range(2, n):
        if n % f == 0:
            fsum += f

    return fsum


def solution():
    return sum([x for x in range(1, 10000) if x != d(x) and x == d(d(x))])

print("The sum of all amicable numbers under 10,000 is {}".format(solution()))
