;;; binary.el --- Binary exercise (exercism)

(require 'seq)

;;; Commentary:

;;; Code:

(defun to-decimal (s)
  (condition-case nil
      (apply '+ (seq-map-indexed
                 (lambda (v i)
                   (cond ((= v ?0) 0)
                         ((= v ?1) (expt 2 i))
                         (t (error "Invalid binary number"))))
                 (seq-reverse s)))
    (error 0)))

(provide 'binary)
;;; binary.el ends here
