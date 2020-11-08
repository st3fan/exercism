(ns sum-of-multiples)

;; I guess I just love to use loop/recur?

(defn- multiple-of [numbers n]
  (loop [numbers numbers]
    (if (empty? numbers)
      0
      (if (zero? (mod n (first numbers)))
        n
        (recur (rest numbers))))))

(defn sum-of-multiples [numbers n]
  (loop [n (dec n) r 0]
    (if (zero? n)
      r
      (recur (dec n) (+ r (multiple-of numbers n))))))
