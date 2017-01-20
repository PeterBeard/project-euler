/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #3
 *
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 */

#include <stdio.h>

// Find the largest prime factor of n
long int largest_prime_factor(long int n)
{
    long int f = 2;
    while(n > 1 && f <= n)
    {
        if(n % f == 0)
        {
            n /= f;
        } else {
            f++;
        }
    }
    return f;
}

int main()
{
    const long int n = 600851475143;
    printf("The largest prime factor of %ld is %ld.\n", n, largest_prime_factor(n));
}
