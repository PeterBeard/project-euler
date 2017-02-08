-------------------------------------------------------------------------------
-- Copyright Peter Beard, Licensed under the GPLv2, see LICENSE for details.
--
-- Problem #4
--
-- A palindromic number reads the same both ways. The largest palindrome made
-- from the product of two 2-digit numbers is 9009 = 91 × 99.
--
-- Find the largest palindrome made from the product of two 3-digit numbers.
-------------------------------------------------------------------------------

with Ada.Text_IO; use Ada.Text_IO;
with Ada.Strings.Fixed;
with Ada.Strings.Unbounded; use Ada.Strings.Unbounded;

procedure Problem004 is
    function IsPalindrome (n : Natural) return Boolean is
        str : Unbounded_String;
        rev : Unbounded_String;
    begin
        str := To_Unbounded_String (Ada.Strings.Fixed.Trim (n'Img, Ada.Strings.Left));
        rev := str;
        -- Reverse the string
        for i in 1 .. Length (str) loop
            Replace_Element (rev, Length (str) - i + 1, Element (str, i));
        end loop;
        -- If the original string is the same as its mirror image then it's a
        -- palindrome
        return str = rev;
    end IsPalindrome;

    max_palindrome : Natural := 0;
begin
    for n in 100 .. 999 loop
        for m in n .. 999 loop
            if n * m > max_palindrome and IsPalindrome (n * m) then
                max_palindrome := m * n;
            end if;
        end loop;
    end loop;
    Put_Line ("The largest palindrome product of two 3-digit numbers is" & max_palindrome'Img);
end Problem004;
