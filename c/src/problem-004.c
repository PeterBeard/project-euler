/*
 * Copyright 2016 Peter Beard
 * Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
 *
 * Problem #4
 *
 * A palindromic number reads the same both ways. The largest palindrome made
 * from the product of two 2-digit numbers is 9009 = 91 × 99.
 *
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */
#include <stdio.h>
#include <stdbool.h>

// Determine whether a number is a palindrome
bool is_palindrome(int n)
{
    // Two three digits numbers will only ever have a 6-digit product
    char n_str[6];
    int len = sprintf(n_str, "%d", n);
    for(int i = 0; i < len; i++)
    {
        if(n_str[i] != n_str[len-i-1])
            return false;
    }
    return true;
}

int main()
{
    int max = 0;
    for(int x = 100; x < 1000; x++)
    {
        for(int y = 100; y < 1000; y++)
        {
            if(x*y > max && is_palindrome(x*y))
                max = x*y;
        }
    }
    printf("The largest palindrome product of two three-digit numbers is %d\n", max);
}
