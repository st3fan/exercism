;;; trinary.el --- Trinary (exercism)

;;; Commentary:

;;; Code:

(require 'subr-x)
(require 'seq)

(defun trinary-digit-p (c)
  (and (>= c ?0) ( <= c ?3)))

(defun trinary-digits (n)
  "Return a list containing all the digits in string N as numbers."
  (let ((digits (string-to-list n)))
    (if (seq-every-p #'trinary-digit-p digits)
        (mapcar (lambda (d)
                  (string-to-number (string d)))
                digits))))

(defun trinary-to-decimal (n)
  (if-let ((digits (trinary-digits n)))
      (seq-reduce #'+ (seq-map-indexed
                       (lambda (d i)
                         (* d (expt 3 i)))
                       (reverse digits)) 0)
    0))

(provide 'trinary)
;;; trinary.el ends here
