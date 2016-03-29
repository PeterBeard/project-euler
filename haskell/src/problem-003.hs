-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- Problem #3
--
-- The prime factors of 13195 are 5, 7, 13 and 29.
-- 
-- What is the largest prime factor of the number 600851475143 ?

-- Get the square root of an Int (rounded up)
isqrt :: Int -> Int
isqrt = ceiling . sqrt . fromIntegral

-- Get all of the prime factors of n
-- Repeated factors will be repeated in the output, e.g. 4 -> [2, 2]
prime_factorization :: Int -> [Int]
prime_factorization 1 = [1]
prime_factorization n =
    case factors of
        [] -> [n]   -- n is prime
        _  -> factors ++ prime_factorization (div n (head factors))
    where factors = take 1 $ filter(\x -> mod n x == 0) [2..isqrt(n)]

solution = last $ prime_factorization 600851475143 
main = putStrLn $ "The greatest prime factor of 600,851,475,143 is " ++ (show solution)
