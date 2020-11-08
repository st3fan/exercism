(ns phone-number)

(def ^:private phone-number-formats
  [#"1?(\d{10})"
   #"1?\((\d{3})\) (\d{3})-(\d{4})"
   #"1?\((\d{3})\)-(\d{3})-(\d{4})"
   #"1?(\d{3})-(\d{3})-(\d{4})"
   #"1?(\d{3})\.(\d{3})\.(\d{4})"])

(def ^:private invalid-number "0000000000")

(defn number [num-string]
  (loop [formats phone-number-formats]
    (if (empty? formats)
      invalid-number
      (if-let [matches (re-matches (first formats) num-string)]
        (clojure.string/join (drop 1 matches))
        (recur (rest formats))))))

(defn area-code [num-string]
  (subs (number num-string) 0 3))

(defn pretty-print [num-string]
  (let [num (number num-string)]
    (str "(" (subs num 0 3) ") " (subs num 3 6) "-" (subs num 6 10))))
