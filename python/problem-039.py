"""
Copyright 2016 Peter Beard
Distributed under the GNU GPL v2. For full terms, see the LICENSE file.

Problem #39

If p is the perimeter of a right angle triangle with integral length sides,
{a,b,c}, there are exactly three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?
"""
def count_triangles(perimeter):
    count = 0
    for c in range(5, round(perimeter/2)):
        for b in range(4, c):
            a = perimeter - (b + c)
            if a > 0 and a*a + b*b == c*c:
                count += 1
    return count

def solution():
    max_solutions = 0
    max_p = 0
    for p in range(12, 1001):
        c = count_triangles(p)
        if c > max_solutions:
            max_solutions = c
            max_p = p

    return max_p

print("The solution is {}".format(solution()))
