/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #10
 *
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 *
 * Find the sum of all the primes below two million.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Use a prime sieve to find all primes less than pmax. Returns an array of all
// the numbers up to pmax with n[i] = 1 if i is prime and 0 otherwise.
int* prime_sieve(int pmax)
{
    int *numbers = (int *) malloc(sizeof(int) * pmax);
    // Initialize everything to prime so we can just mark the composite numbers
    memset(numbers, 1, sizeof(int) * pmax);
    // We know 0 and 1 aren't prime
    numbers[0] = 0;
    numbers[1] = 0;
    // Sieve of Eratosthenes
    for(int n = 2; n < pmax; n++)
    {
        for(int i = 2*n; i < pmax; i += n)
        {
            numbers[i] = 0;
        }
    }
    return numbers;
}

int main()
{
    const int pmax = 2000000;
    long int sum = 0;

    int *primes = prime_sieve(pmax);
    for(int i = 0; i < pmax; i++)
    {
        if(primes[i])
            sum += i;
    }
    printf("The sum of all primes less than 2,000,000 is %ld.\n", sum);
}
