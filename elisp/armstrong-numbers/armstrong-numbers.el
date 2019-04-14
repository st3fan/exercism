;;; armstrong-numbers.el --- armstrong-numbers Exercise (exercism)

;;; Commentary:

;;; Code

(require 'seq)

(defun digits (n)
  "Return a list containing all the digits in N as numbers."
  (mapcar (lambda (d)
            (string-to-number (string d)))
          (string-to-list (number-to-string n))))

(defun armstrong (n)
  "Calculate the Armstrong number for N."
  (let* ((digits (digits n)) (length (length digits)))
    (seq-reduce #'+ (mapcar (lambda (d) (expt d length)) digits) 0)))

(defun armstrong-p (n)
  "Determine if N is an Armstrong number."
  (= (armstrong n) n))

(provide 'armstrong-numbers)
;;; armstrong-numbers.el ends here
