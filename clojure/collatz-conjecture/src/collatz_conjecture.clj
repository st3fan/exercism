(ns collatz-conjecture)

(defn collatz
  "Return the Collatz count for the given number. Throws an
   exception if the number if zero or negative."
  [num]
  {:pre [(> num 0)]}
  (loop [n num c 0]
    (if (= n 1)
      c
      (if (even? n)
        (recur (/ n 2) (inc c))
        (recur (inc (* n 3)) (inc c))))))
