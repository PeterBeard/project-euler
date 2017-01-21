/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #7
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
 * that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 */
#include <stdio.h>

// Determine whether a number is prime - 1 for prime, 0 for not prime
int is_prime(int n)
{
    int f = 2;
    while(f*f <= n)
    {
        if(n % f == 0)
            return 0;
        f++;
    }
    return 1;
}

// Find the nth prime number
int nth_prime(int n)
{
    if(n <= 1)
        return 2;

    int p = 1;
    while(n > 0)
    {
        p++;
        if(is_prime(p))
            n--;
    }
    return p;
}

int main()
{
    printf("The 10,001st prime is %d.\n", nth_prime(10001));
}
