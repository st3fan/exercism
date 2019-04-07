;;; difference-of-squares.el --- Difference of Squares (exercism)

;;; Commentary:

;;; Code:

(defun square-of-sum-accumulator (n a)
  (if (zerop n)
      a
    (square-of-sum-accumulator (1- n) (+ a n))))

(defun square-of-sum (n)
  (let ((sum (square-of-sum-accumulator n 0)))
    (* sum sum)))

(defun sum-of-squares-accumulator (n a)
  (if (zerop n)
      a
    (sum-of-squares-accumulator (1- n) (+ a (* n n)))))

(defun sum-of-squares (n)
  (sum-of-squares-accumulator n 0))

(defun difference (n)
  (- (square-of-sum n) (sum-of-squares n)))

(provide 'difference-of-squares)
;;; difference-of-squares.el ends here
