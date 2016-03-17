-- Copyright 2016 Peter Beard
-- Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
--
-- A Pythagorean triplet is a set of three natural numbers, a < b < c, for
-- which,
-- a^2 + b^2 = c^2
-- 
-- For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
-- 
-- There exists exactly one Pythagorean triplet for which a + b + c = 1000.
-- Find the product abc.

-- Find the hypotenuse of a triangle with legs a and b
hypotenuse :: Floating a => a -> a -> a
hypotenuse a b = sqrt(a^2 + b^2)

-- Determine whether two numbers could be the legs of a Pythagorean triple
areLegs :: (Ord a, RealFrac a, Floating a) => a -> a -> Bool
areLegs a b
    | a >= b = False
    | otherwise = (ceiling(c) == floor(c))
        where c = hypotenuse a b

-- Find Pythagorean pairs (a and b) in the range [1..n]
pairs :: (RealFrac a, Floating a, Enum a) => a -> [(a, a)]
pairs n = concat $ filter (not . null) $ map (\a -> zip (repeat a) $ filter (\b -> areLegs a b) [a..n]) [1..n]

solution = round $ a * b * hypotenuse a b
    where (a, b) = head $ filter (\(a, b) -> a + b + (hypotenuse a b) == 1000) $ pairs 1000


main = putStrLn $ "The product of the triple where a + b + c = 1,000 is " ++ (show solution)
