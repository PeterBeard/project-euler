-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- Problem #10
--
-- The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
-- 
-- Find the sum of all the primes below two million.

-- Get the square root of an Int (rounded up)
isqrt :: Int -> Int
isqrt = ceiling . sqrt . fromIntegral

-- Determine whether an integer is prime
isPrime :: Int -> Bool
isPrime n
    | n < 2 = False
    | n == 2 = True
    | otherwise = not $ any (\f -> mod n f == 0) [2..isqrt(n)]

solution = sum $ filter isPrime [2..2000000]

main = putStrLn $ "The sum of all primes < 2,000,000 is " ++ (show solution)
