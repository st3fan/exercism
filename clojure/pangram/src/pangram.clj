(ns pangram)

(defn unique-letters
  "Return a set containing the letters in s."
  [s]
  (->> s
       (filter #(Character/isLetter %))
       (map #(Character/toLowerCase %))
       (into #{})))

(defn pangram?
  "Return true if the given string is a pangram."
  [s]
  (= 26 (count (unique-letters s))))
