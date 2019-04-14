;;; nucleotide-count.el --- nucleotide-count Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'cl)

(defun nucleotide-count (dna-strain)
  (let ((result (list (cons ?A 0) (cons ?C 0) (cons ?G 0) (cons ?T 0))))
    (mapc (lambda (n)
            (if (or (= n ?A) (= n ?C) (= n ?G) (= n ?T))
                (cl-incf (alist-get n result))
              (error "Invalid nucleotide in strain")))
          (string-to-list dna-strain))
    result))

(provide 'nucleotide-count)
;;; nucleotide-count.el ends here
