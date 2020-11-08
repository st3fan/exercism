(ns octal)

(defn- exp [x y]
  (loop [y y a 1]
    (if (zero? y)
      a
      (recur (dec y) (* a x)))))

(defn to-decimal [s]
  (loop [s s
         n (dec (count s))
         r 0]
    (if (empty? s)
      r
      (if-not (Character/isDigit (first s))
        0
        (recur
         (rest s)
         (dec n)
         (+ r (* (Integer/parseInt (str (first s))) (exp 8 n))))))))
