/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #6
 *
 * The sum of the squares of the first ten natural numbers is,
 * 1^2 + 2^2 + ... + 10^2 = 385
 *
 * The square of the sum of the first ten natural numbers is,
 * (1 + 2 + ... + 10)^2 = 552 = 3025
 *
 * Hence the difference between the sum of the squares of the first ten natural
 * numbers and the square of the sum is 3025 − 385 = 2640.
 *
 * Find the difference between the sum of the squares of the first one hundred
 * natural numbers and the square of the sum.
 */
#include <stdio.h>

int main()
{
    int sum = 0;
    int sum_of_squares = 0;
    for(int n = 1; n <= 100; n++)
    {
        sum += n;
        sum_of_squares += (n*n);
    }
    printf("The difference between the sums is %d.\n", (sum*sum - sum_of_squares));
}
