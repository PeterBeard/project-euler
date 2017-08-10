;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #5
;;
;; 2520 is the smallest number that can be divided by each of the numbers from 1 to
;; 10 without any remainder.
;; 
;; What is the smallest positive number that is evenly divisible by all of the
;; numbers from 1 to 20?

(ns problem-005.core
  (:gen-class))

(defn div-upto?
  "Determine whether a number is divisible by every integer up to some maximum"
  [limit n]
  (if (or (= n 0) (< n limit))
    false
    (if (<= limit 1)
      true
      (if (= 0 (mod n limit))
        (recur (dec limit) n)
        false))))

(defn solution
  "Solution for problem 5"
  []
  (first (filter (partial div-upto? 20) (range))))

(defn -main
  "Print the solution"
  [& args]
  (println "The smallest positive integer divisible by all of [1,20] is" (solution)))
