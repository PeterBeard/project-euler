-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- Problem #1
--
-- If we list all the natural numbers below 10 that are multiples of 3 or 5,
-- we get 3, 5, 6 and 9. The sum of these multiples is 23.
--
-- Find the sum of all the multiples of 3 or 5 below 1000. 
solution = sum $ filter (\n -> mod n 3 == 0 || mod n 5 == 0) [1..999]
main = putStrLn $ "The sum of all multiples of 3 or 5 below 1,000 is " ++ (show solution)
