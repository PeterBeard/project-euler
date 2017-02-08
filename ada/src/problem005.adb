-------------------------------------------------------------------------------
-- Copyright Peter Beard, Licensed under the GPLv2, see LICENSE for details.
--
-- Problem #5
-- 
-- 2520 is the smallest number that can be divided by each of the numbers from 1
-- to 10 without any remainder.
--
-- What is the smallest positive number that is evenly divisible by all of the
-- numbers from 1 to 20?
--
-------------------------------------------------------------------------------

with Ada.Text_IO; use Ada.Text_IO;

procedure Problem005 is
    -- Determine whether n is divisible by all of [2, max_factor]
    function DivisibleUpTo (max_factor : Natural; n : Natural) return Boolean is
    begin
        for f in reverse 2 .. max_factor loop
            if n mod f /= 0 then
                return False;
            end if;
        end loop;
        return True;
    end DivisibleUpTo;

    n : Natural := 2520;
begin
    while not DivisibleUpTo (20, n) loop
        n := n + 20;
    end loop;

    Put_Line ("The smallest number divisible by 1 - 20 is" & n'Img);
end Problem005;
