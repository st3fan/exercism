;;; crypto-square.el --- Crypto Square (exercism)

;;; Commentary:

;;; Code:

(require 'seq)
(require 'subr-x)

(defun square--normalize (s)
  (seq-filter (lambda (c) (or (and (>= c ?0) (<= c ?9)) (and (>= c ?a) (<= c ?z)))) (downcase s)))

(defun square--encode (normalized c)
  (seq-partition normalized c))

(defun square--pad (encoded r)
  (seq-map (lambda (chunk) (if (< (length chunk) r)
                               (append chunk (string-to-list (make-string (1+ (- r (length chunk))) ?\s)))
                             chunk)) encoded))

(defun square--format (encoded)
  (apply #'seq-mapn (lambda (&rest values) (concat values)) encoded))

(defun encipher (plaintext)
  (let ((normalized (square--normalize plaintext)))
    (if (zerop (length normalized)) ;; TODO This is not great
        ""
      (let* ((n (length normalized))
             (c (ceiling (sqrt n)))
             (r (ceiling (/ n (sqrt n)))))
        (thread-first normalized
          (square--encode c)
          (square--pad r)
          (square--format)
          (string-join " "))))))

(provide 'crypto-square)
;;; crypto-square.el ends here
