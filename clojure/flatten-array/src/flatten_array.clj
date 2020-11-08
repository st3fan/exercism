(ns flatten-array)

;; Using clojure.core is always simpler.
(defn flatten-simple [arr]
  (filter (complement nil?) (clojure.core/flatten arr)))

;; This is with just 'primitives'.
(defn flatten [arr]
  (loop [arr arr acc []]
    (if (empty? arr)
      acc
      (let [e (first arr) r (rest arr)]
        (if (vector? e)
          (recur r (into [] (concat acc (flatten e))))
          (recur r (if (nil? e) acc (conj acc e))))))))
