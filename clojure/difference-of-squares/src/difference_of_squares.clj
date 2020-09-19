(ns difference-of-squares)

(defn sum-of-squares [n]
  (reduce + (map #(* %1 %1) (range 1 (+ 1 n)))))

(defn square-of-sum [n]
  (let [sum (reduce + (range 1 (+ 1 n)))]
    (* sum sum)))

(defn difference [n]
  (- (square-of-sum n) (sum-of-squares n)))
