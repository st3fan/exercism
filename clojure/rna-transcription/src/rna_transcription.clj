(ns rna-transcription)

(def dna->rna-mappings
  {\G \C \C \G \T \A \A \U})

(defn dna->rna [dna]
  (if-let [rna (dna->rna-mappings dna)]
    rna
    (throw (AssertionError. "Invalid DNA nucleotide."))))

(defn to-rna [dna]
  (apply str (map dna->rna dna)))
