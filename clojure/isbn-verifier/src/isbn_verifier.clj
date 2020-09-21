(ns isbn-verifier)

(defn- isbn-digit-to-int [digit]
  (if (= digit "X")
    10
    (Integer/parseInt digit)))

(defn- isbn-check [digits]
  (let [length (count digits)]
    (reduce + (map-indexed #(* %2 (- length %1)) digits))))

(defn isbn? [isbn]
  (if-let [digits (re-matches #"(\d)-?(\d)(\d)(\d)-?(\d)(\d)(\d)(\d)(\d)-?(\d|X)" isbn)]
    (zero? (mod (isbn-check (map isbn-digit-to-int (drop 1 digits))) 11))
    false))
