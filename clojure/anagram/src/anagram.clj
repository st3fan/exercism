(ns anagram
  (:require [clojure.string :refer [lower-case]]))

(defn anagrams-for [word prospect-list]
  (let [sorted (sort (lower-case word))]
    (filter #(and
              (not= (lower-case word) (lower-case %))
              (= sorted (sort (lower-case %))))
            prospect-list)))
