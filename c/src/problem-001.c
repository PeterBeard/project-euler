/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #1
 *
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get
 * 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 *
 */
#include <stdio.h>

int main()
{
    int sum = 0;
    for(int i = 0; i < 1000; i++)
    {
        if(i % 3 == 0 || i % 5 == 0)
            sum += i;
    }
    printf("The sum of the multiples of 3 or 5 under 1,000 is %d\n", sum);
}
