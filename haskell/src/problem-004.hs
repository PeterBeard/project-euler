-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- A palindromic number reads the same both ways. The largest palindrome made
-- from the product of two 2-digit numbers is 9009 = 91 × 99.
-- 
-- Find the largest palindrome made from the product of two 3-digit numbers.

-- Get the number of digits in an integer
numDigits :: Int -> Int
numDigits n = floor $ logBase 10.0 $ fromIntegral n

-- Take the last digit of an integer and make it the first
invertDigit :: Int -> Int
invertDigit n = (mod n 10) * (10 ^ numDigits(n))

-- Reverse the digits of an integer
reverseDigits :: Int -> Int
reverseDigits n
    | n < 10 = n
    | otherwise = invertDigit n + reverseDigits(div n 10)

-- Determine whether an integer is a palindrome
isPalindrome :: Int -> Bool
isPalindrome n = n == reverseDigits n

solution = maximum $ maximum $ map (\x -> filter isPalindrome $ zipWith (*) (repeat x) [100..999]) [100..999]

main = putStrLn $ "The solution is " ++ (show solution)
