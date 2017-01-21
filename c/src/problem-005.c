/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #5
 *
 * 2520 is the smallest number that can be divided by each of the numbers from 1
 * to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the
 * numbers from 1 to 20?
 */
#include <stdio.h>

int main()
{
    int n = 20;
    while(1)
    {
        n += 20;
        int i = 20;
        for(i = 20; i > 1; i--)
        {
            if(n % i != 0)
                break;
        }
        if(i == 1)
            goto end;
    }
    end:printf("The smallest number divisible by 1 - 20 is %d\n", n);
}
