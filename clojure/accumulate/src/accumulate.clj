(ns accumulate)

(defn accumulate
  [fn coll]
  (loop [items coll results []]
    (if (empty? items)
      results
      (recur (rest items)
             (conj results (fn (first items)))))))
