"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

You are given the following information, but you may prefer to do some research
for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century
        unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1
Jan 1901 to 31 Dec 2000)?
"""

def day_of_week(day, month, year):
    """
    Determine the day of the week that the given date falls on.
    day is the day of the month starting from 1
    month is the number of the month starting with 1 for January, 2 for February, etc.
    year is a 4-digit year
    Returns 0 for Sunday, 1 for Monday, etc.
    Formula from https://en.wikipedia.org/wiki/Determination_of_the_day_of_the_week
    """
    m_values = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4]
    c_values = [0, 5, 3, 1]

    m = m_values[month - 1]

    y = year % 100
    if month == 1 or month == 2:
        y = (year - 1) % 100
    y = (y + y//4) % 7
    c = c_values[(year//100) % 4]

    return (day + m + y + c) % 7

def solution():
    count = 0
    for m in range(1, 13):
        for y in range(1901, 2001):
            if day_of_week(1, m, y) == 0:
                count += 1
    return count

print("The number of first Sundays between 1901-01-01 and 2000-12-31 is {}".format(solution()))
