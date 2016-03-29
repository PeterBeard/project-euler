"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #022

Using names.txt (right click and 'Save Link/Target As...'), a 46K text file
containing over five-thousand first names, begin by sorting it into alphabetical
order. Then working out the alphabetical value for each name, multiply this
value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is
worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would
obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
"""

def score_name(name):
    """
    Calculate a name's score.

    Each letter is assigned a value 1-26 where A = 1, B = 2, etc.
    The score is the sum of the value of each letter in the name.
    """
    return sum([(ord(c) - 64) for c in name.upper()])


def load_name_file(filename):
    """Load a list of names from the given file and return it."""
    with open(filename, 'r') as fh:
        return [n.replace('"', '') for n in fh.readline().split(",")]


def solution():
    names = sorted(load_name_file("../data/p022_names.txt"))
    total = 0
    for i in range(0, len(names)):
        total += (i + 1) * score_name(names[i])

    return total


print("The total weighted score of all names when alphabetized is {}".format(solution()))
