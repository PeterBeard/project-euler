"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #031

In England the currency is made up of pound, £, and pence, p, and there are
eight coins in general circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

It is possible to make £2 in the following way:

    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number of coins?
"""

def calc_val(one_pound, fifty_p, twenty_p, ten_p, five_p, two_p, one_p=0):
    return one_pound * 100 +\
           fifty_p * 50 + \
           twenty_p * 20 + \
           ten_p * 10 + \
           five_p * 5 + \
           two_p * 2 + \
           one_p

def solution():
    # Uncovered possiblities are one two-pound coin and 200 1p coins
    count = 2
    for one in range(0, 2):
        value = calc_val(one, 0, 0, 0, 0, 0)
        for fifty in range(0, (200 - value)//50 + 1):
            value = calc_val(one, fifty, 0, 0, 0, 0)
            for twenty in range(0, (200 - value)//20 + 1):
                value = calc_val(one, fifty, twenty, 0, 0, 0)
                for ten in range(0, (200 - value)//10 + 1):
                    value = calc_val(one, fifty, twenty, ten, 0, 0)
                    for five in range(0, (200 - value)//5 + 1):
                        value = calc_val(one, fifty, twenty, ten, five, 0)
                        for two in range(0, (200 - value)//2 + 1):
                            value = calc_val(one, fifty, twenty, ten, five, two)
                            count += 1
    return count

print("The number of ways to make two pounds is {}".format(solution()))
