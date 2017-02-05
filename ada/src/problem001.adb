-------------------------------------------------------------------------------
-- Copyright Peter Beard, Licensed under the GPLv2, see LICENSE for details.
--
-- Problem #1
--
-- If we list all the natural numbers below 10 that are multiples of 3 or 5, we
-- get 3, 5, 6 and 9. The sum of these multiples is 23.
--
-- Find the sum of all the multiples of 3 or 5 below 1000.
-------------------------------------------------------------------------------

with Ada.Text_IO; use Ada.Text_IO;

procedure Problem001 is
    sum : Integer := 0;
begin
    for n in 3 .. 999 loop
        if n mod 3 = 0 or n mod 5 = 0 then
            sum := sum + n;
        end if;
    end loop;
    Put_Line("The sum of all multiples of 3 or 5 below 1000 is" & Integer'Image(sum));
end Problem001;
