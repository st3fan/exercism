(ns nucleotide-count)

(def nucleotide-keys (set "ACTG"))

(defn count-of-nucleotide-in-strand [nucleotide strand]
  {:pre [(nucleotide-keys nucleotide)]}
  (count (filter #(= % nucleotide) strand)))

(defn nucleotide-counts [strand]
  (merge {\A 0 \C 0 \T 0 \G 0} (frequencies strand)))
