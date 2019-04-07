;;; hamming.el --- Hamming (exercism)

(require 'cl)

;;; Commentary:

;;; Code:

(defun hamming-distance (a b)
  (if (eq (length a) (length b))
      (let ((a (string-to-list a)) (b (string-to-list b)))
        (count nil (cl-mapcar 'eq a b)))
    (error "strains are not equal length")))

(provide 'hamming)
;;; hamming.el ends here
