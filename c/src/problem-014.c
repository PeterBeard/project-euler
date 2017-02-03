/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #14
 *
 * The following iterative sequence is defined for the set of positive integers:
 *
 * n → n/2 (n is even)
 * n → 3n + 1 (n is odd)
 *
 * Using the rule above and starting with 13, we generate the following
 * sequence:
 * 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 *
 * It can be seen that this sequence (starting at 13 and finishing at 1)
 * contains 10 terms. Although it has not been proved yet (Collatz Problem), it
 * is thought that all starting numbers finish at 1.
 *
 * Which starting number, under one million, produces the longest chain?
 *
 * NOTE: Once the chain starts the terms are allowed to go above one million.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_MAX 1000000

// Calculate the length of the Collatz chain starting with n
// Note that n must be less than the size of the memo array
// Also note that the lengths array must be initialized to lengths[1] = 1 since
// we assume that the Collatz conjecture is true
//
// Returns -1 if the array is initialized wrong or n overflows, otherwise
// returns the length of the chain starting at n, e.g. if n = 13, return 10
int collatz_length(int orig_n, int *lengths)
{
    int length = 0;
    long int n = orig_n;
    while(n > 0)
    {
        if(n < N_MAX && lengths[n] != 0)
        {
            length += lengths[n];
            lengths[orig_n] = length;
            return length;
        }
        if(n % 2 == 0)
        {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length++;
    }
    return -1;
}

int main()
{
    int *lengths = malloc(N_MAX * sizeof(int));
    memset(lengths, 0, N_MAX * sizeof(int));
    lengths[1] = 1;

    int max_length = 0;
    int longest_n = 0;

    for(int n = 1; n < N_MAX; n++)
    {
        int length = collatz_length(n, lengths);
        if(length > max_length)
        {
            max_length = length;
            longest_n = n;
        }
    }

    printf("The number under 1,000,000 with the longest Collatz sequence is %d.\n", longest_n);
    free(lengths);
}
