;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #4
;;
;; A palindromic number reads the same both ways. The largest palindrome made from
;; the product of two 2-digit numbers is 9009 = 91 × 99.
;; 
;; Find the largest palindrome made from the product of two 3-digit numbers.

(ns problem-004.core
  (:gen-class))

(defn palindrome?
  "Determine whether a number is a palindrome"
  [n]
  (= (str n) (apply str (reverse (str n)))))

(defn solution
  "Solution for problem 4"
  []
  (apply max (filter palindrome? 
                     (for [x (range 100 1000) y (range 100 1000)] (* x y)))))

(defn -main
  "Print the solution"
  [& args]
  (println "The largest palindrome made from the product of two three-digit numbers is" (solution)))
