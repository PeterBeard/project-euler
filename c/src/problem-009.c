/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #9
 *
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for
 * which a^2 + b^2 = c^2
 *
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 *
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 */
#include <stdio.h>

// Determine whether a, b, and c for a Pythagorean triplet. Returns 1 if they do
// and 0 if they don't.
int is_triplet(int a, int b, int c)
{
    if(a*a + b*b == c*c)
        return 1;
    return 0;
}

int main()
{
    const int target = 1000;
    int product = 0;
    for(int a = 1; a < 1000; a++)
    {
        for(int b = 1; b < 1000; b++)
        {
            for(int c = 1; c < 1000; c++)
            {
                if(a + b + c == target && is_triplet(a, b, c))
                {
                    product = a*b*c;
                    goto exit;
                }
            }
        }
    }
    exit: printf("The product of the triplet with sum %d is %d.\n", target, product);
}
