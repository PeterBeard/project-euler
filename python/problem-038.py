"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #038

Take the number 192 and multiply it by each of 1, 2, and 3:

    192 × 1 = 192
    192 × 2 = 384
    192 × 3 = 576

By concatenating each product we get the 1 to 9 pandigital, 192384576. We will
call 192384576 the concatenated product of 192 and (1,2,3)

The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and
5, giving the pandigital, 918273645, which is the concatenated product of 9 and
(1,2,3,4,5).

What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
concatenated product of an integer with (1,2, ... , n) where n > 1?
"""
from math import log10

def is_pandigital_str(s):
    if len(s) != 9:
        return False

    return all([str(c) in s for c in range(1, 10)])

def concat_product_str(n, values):
    return ''.join([str(n * v) for v in values])

def solution():
    max_product = 0
    # Anything with more than 4 digits will have a 10-digit product or more so
    # we only need to check up to 10^4
    for n in range(1, 10**4):
        # We only need to try numbers up to about 10 / log(n) since more digits
        # means a longer product
        rangemax = round(10.0 / max(log10(n), 1))
        for k in range(2, rangemax):
            product = concat_product_str(n, range(1, k+1))
            if len(product) == 9 and is_pandigital_str(product):
                max_product = max(max_product, int(product))

    return max_product

print("The solution is {}".format(solution()))
