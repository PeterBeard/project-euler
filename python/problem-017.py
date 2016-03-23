"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

If the numbers 1 to 5 are written out in words: one, two, three, four, five,
then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in
words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and
forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20
letters. The use of "and" when writing out numbers is in compliance with British
usage.
"""

def write_int(n):
    """Express the integer n in English"""
    digits = [
        'zero',
        'one',
        'two',
        'three',
        'four',
        'five',
        'six',
        'seven',
        'eight',
        'nine'
    ]
    teens = [
        'ten',
        'eleven',
        'twelve',
        'thirteen',
        'fourteen',
        'fifteen',
        'sixteen',
        'seventeen',
        'eighteen',
        'nineteen'
    ]
    tens = [
        'twenty',
        'thirty',
        'forty',
        'fifty',
        'sixty',
        'seventy',
        'eighty',
        'ninety'
    ]
    if n < 10:
        return digits[n]
    elif n < 20:
        return teens[n - 10]
    elif n < 100:
        ten = n//10
        if n % 10 == 0:
            return tens[ten - 2]
        else:
            return tens[ten - 2] + "-" + write_int(n - ten*10)
    elif n < 1000:
        hundreds = n//100
        if n % 100 == 0:
            return write_int(hundreds) + " hundred"
        else:
            return write_int(hundreds) + " hundred and " + \
                write_int(n - (100*hundreds))
    return "one thousand"


def solution():
    count = 0
    for n in range(1,1001):
        count += len(write_int(n).replace(' ','').replace('-', ''))
    return count

print("The total number of letters in all integers between 1 and 1000 is {}".format(solution()))
