(ns armstrong-numbers)

;; Because this keeps coming back in exercises, I turned it into a
;; small function.

(defn digits-in-number [n]
  (loop [n n digits '()]
    (if (zero? n)
      digits
      (recur (quot n 10) (conj digits (mod n 10))))))

;; Previously I used Math/pow but that fails on big numbers. This will
;; simply "upgrade" numbers to bignums if they don't fit into builtins
;; anymore.

(defn pow [x e]
  (loop [n e a 1]
    (if (zero? n)
      a
      (recur (dec n) (* a x)))))

;; I keep coming back to nested map/reduce. I'm fine with this but I
;; see a lot of other people use things like the threading macros.

(defn armstrong? [num]
  (let [digits (digits-in-number num) length (count digits)]
    (= num (reduce + (map #(pow % length) digits)))))
