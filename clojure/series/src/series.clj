(ns series)

(defn slices [string length]
  (if (zero? length)
    [""] ;; Is there no nicer way to handle this weird edge case?
    (loop [s string r []]
      (if (> length (count s))
        r
        (recur (subs s 1) (conj r (subs s 0 length)))))))
