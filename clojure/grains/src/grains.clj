(ns grains)

;; The trick with this exercise was to make sure Clojure would use
;; BigInteger values. The "2N" in square makes sure results are using
;; that integer type. Maybe there is a more elegant way to do that?

(defn pow [x e]
  (loop [n e a 1]
    (if (zero? n)
      a
      (recur (dec n) (* a x)))))

(defn square [position]
  (pow 2N (dec position)))

(defn total []
  (reduce + (map square (range 1 65))))
