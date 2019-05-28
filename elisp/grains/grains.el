;;; grains.el --- Grains exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)

(defun square (n)
  (expt 2 (float (1- n))))

(defun total ()
  (seq-reduce #'+ (seq-map #'square (number-sequence 1 64)) 0))

(provide 'grains)
;;; grains.el ends here
