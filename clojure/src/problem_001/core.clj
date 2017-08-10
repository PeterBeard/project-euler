;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #1
;;
;; If we list all the natural numbers below 10 that are multiples of 3 or 5, we get
;; 3, 5, 6 and 9. The sum of these multiples is 23.
;;
;; Find the sum of all the multiples of 3 or 5 below 1000.

(ns problem-001.core
  (:gen-class))

(defn multiple35?
  "Determine whether a number is a multiple of three or five"
  [n]
  (or (= 0 (mod n 3)) (= 0 (mod n 5))))

(defn solution
  "Solution for problem 1"
  []
  (reduce + (filter multiple35? (range 0 1000))))

(defn -main
  "Print the solution"
  [& args]
  (println "The sum of the multiples of 3 or 5 under 1,000 is" (solution)))
