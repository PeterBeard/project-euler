-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- 2520 is the smallest number that can be divided by each of the numbers from 1
-- to 10 without any remainder.
-- 
-- What is the smallest positive number that is evenly divisible by all of the
-- numbers from 1 to 20?

-- Determine whether an integer is divisible by all integers in the given list
divisibleBy :: [Int] -> Int -> Bool
divisibleBy xs n = all (\f -> mod n f == 0) xs

solution = head $ filter (divisibleBy [20,19..11]) [20,40..]

main = putStrLn $ "The solution is " ++ (show solution)
