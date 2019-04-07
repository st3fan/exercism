;;; perfect-numbers.el --- perfect-numbers Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)

(defun aliquot-factors (number)
  "Returns the aliquot factors for NUMBER. Exludes negative factors and itself."
  (seq-filter (lambda (n) (zerop (mod number n))) (number-sequence 1 (/ number 2))))

(defun aliquot-sum (number)
  "Calculates the Aliquot sum for NUMBER."
  (apply #'+ (aliquot-factors number)))

(defun perfect-number-p (number)
  "Tests whether NUMBER is a perfect number."
  (= (aliquot-sum number) number))

(defun perfect-numbers (up-to)
  "Returns all perfect numbers to to UP-TO."
  (seq-filter #'perfect-number-p (number-sequence 1 up-to)))

(provide 'perfect-numbers)
;;; perfect-numbers.el ends here
