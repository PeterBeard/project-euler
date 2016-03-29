"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #20

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
"""
def sum_of_digits(n):
    """Find the sum of the digits in n"""
    dsum = 0
    while n > 0:
        dsum += n % 10
        n //= 10
    return dsum


def factorial(n):
    """Calcualte n!"""
    fact = n
    for m in range(n-1, 1, -1):
        fact *= m
    return fact


def solution():
    return sum_of_digits(factorial(100))

print("The sum of the digits of 100! is {}".format(solution()))
