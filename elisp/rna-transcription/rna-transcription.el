;;; rna-transcription.el -- RNA Transcription (exercism)

;;; Commentary:



;;; Code:

(defconst nucleotide-complements
  '((?G . ?C)
    (?C . ?G)
    (?T . ?A)
    (?A . ?U)))

(defun nucleotide-complement (nucleotide)
  "Return the RNA complement for NUCLEOTIDE"
  (cdr (assoc nucleotide nucleotide-complements)))

(defun to-rna (strand)
  "Return the RNA complement for the DNA strand STRAND"
  (apply #'string (mapcar #'nucleotide-complement strand)))

(provide 'rna-transcription)
;;; rna-transcription.el ends here
