-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- Problem #6
--
-- The sum of the squares of the first ten natural numbers is,
-- 12 + 22 + ... + 102 = 385
-- 
-- The square of the sum of the first ten natural numbers is,
-- (1 + 2 + ... + 10)2 = 552 = 3025
-- 
-- Hence the difference between the sum of the squares of the first ten natural
-- numbers and the square of the sum is 3025 − 385 = 2640.
-- 
-- Find the difference between the sum of the squares of the first one hundred
-- natural numbers and the square of the sum.

solution = abs $ (sum $ map (^2) [1..100]) - (sum [1..100]) ^ 2
main = putStrLn $ "The solution is " ++ (show solution)
