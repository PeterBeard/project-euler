-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
-- that the 6th prime is 13.
-- 
-- What is the 10 001st prime number?

-- Get the square root of an Int (rounded up)
isqrt :: Int -> Int
isqrt = ceiling . sqrt . fromIntegral

-- Determine whether an integer is prime
isPrime :: Int -> Bool
isPrime n
    | n < 2 = False
    | n == 2 = True
    | otherwise = not $ any (\f -> mod n f == 0) [2..isqrt(n)]

solution = filter isPrime [2..] !! 10000
main = putStrLn $ "The solution is " ++ (show solution)
