;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #7
;;
;; By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
;; the 6th prime is 13.
;; 
;; What is the 10 001st prime number?

(ns problem-007.core
  (:gen-class))

(defn prime?
  "Determine whether a number is prime"
  [n]
  (and (> n 1) (or (= n 2)
    (do
      (def lim (+ 2 (int (Math/sqrt n))))
      (not-any? zero? (map (partial mod n) (range 2 lim)))))))

(defn solution
  "Solution for problem 7"
  []
  (nth (filter prime? (range)) 10000))

(defn -main
  "Print the solution"
  [& args]
  (println "The solution is" (solution)))
