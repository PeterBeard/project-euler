;; Copyright 2017 Peter Beard
;; Distributed under the GNU GPL v2. For full terms, see the LICENSE file.
;;
;; Problem #3
;;
;; The prime factors of 13195 are 5, 7, 13 and 29.
;;
;; What is the largest prime factor of the number 600851475143 ?

(ns problem-003.core
  (:gen-class))

(def n 600851475143)

(defn prime-factors
  "List a number's prime factors"
  ([n] (prime-factors n 2))
  ([n fac]
    (if (or (< n 2) (> fac n))
      []
      (if (= 0 (mod n fac))
        (into [fac] (prime-factors (/ n fac) (inc fac)))
        (recur n (inc fac))))))

(defn solution
  "Solution for problem 3"
  []
  (apply max (prime-factors n)))

(defn -main
  "Print the solution"
  [& args]
  (println "The largest prime factor of" n "is" (solution)))
