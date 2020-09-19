(ns binary)

;; I wrote this function to split a number in its digits but then I
;; saw that the tests all use strings. So this is not used.

;; (defn digits [n]
;;   (loop [n n digits '()]
;;     (if (zero? n)
;;       digits
;;       (recur (quot n 10) (conj digits (mod n 10))))))

(defn pow [x n]
  (int (Math/pow x n)))

(defn parse-digits [n]
  (try
    (doall (map #(Long/parseLong (str %)) n))
    (catch NumberFormatException _)))

(defn to-decimal [n]
  (let [digits (parse-digits n)]
    (reduce + (map-indexed #(* %2 (pow 2 %1)) (reverse digits)))))
