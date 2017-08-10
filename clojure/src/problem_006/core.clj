;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #6
;;
;; The sum of the squares of the first ten natural numbers is,
;;               1² + 2² + ... + 10² = 385
;; 
;; The square of the sum of the first ten natural numbers is,
;;             (1 + 2 + ... + 10)² = 55² = 3025
;; 
;; Hence the difference between the sum of the squares of the first ten natural
;; numbers and the square of the sum is 3025 − 385 = 2640.
;; 
;; Find the difference between the sum of the squares of the first one hundred
;; natural numbers and the square of the sum.

(ns problem-006.core
  (:gen-class))

(defn square
  "Square a number"
  [n]
  (* n n))

(defn solution
  "Solution for problem 6"
  []
  (- (square (reduce + (range 101))) (reduce + (map square (range 101)))))

(defn -main
  "Print the solution"
  [& args]
  (println "The difference between the sum of [1,100] squared and the square of their sum is" (solution)))
