(ns leap)

(defn leap-year? [year]
  (and (zero? (mod year 4))
       (or (zero? (mod year 400))
           (not (zero? (mod year 100))))))
