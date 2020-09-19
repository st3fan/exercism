(ns nth-prime)

(defn prime?
  "Determine whether n is prime using a basic trial-division method."
  [x]
  (if (= x 2)
    true
    (loop [d 2]
      (if (zero? (mod x d))
        false
        (if (> (* d d) x)
          true
          (recur (inc d)))))))

(defn primes
  "Generate a lazy sequence of prime numbers"
  []
  (keep #(if (prime? %) %) (iterate inc 1)))

(defn nth-prime [index]
  (if (zero? index)
    (throw (IllegalArgumentException. "there is no zero-th prime")))
  (nth (primes) index))

