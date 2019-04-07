;;; etl.el --- etl Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)

(defun etl (scores)
  (let ((result (make-hash-table)))
    (maphash (lambda (score letters)
               (if (< score 0)
                   (error "Invalid score (negative)")
                 (mapc (lambda (letter)
                         (if (not (stringp letter))
                             (error "Invalid letter (not a string)")
                           (puthash (downcase letter) score result))) letters))) scores)
    result))

(provide 'etl)
;;; etl.el ends here
