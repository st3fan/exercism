(ns word-count
  (:require [clojure.string :refer [lower-case]]))

(defn word-count [s]
  (frequencies (map lower-case (re-seq #"\w+" s))))
