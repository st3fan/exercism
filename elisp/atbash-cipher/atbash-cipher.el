;;; atbash-cipher.el --- Atbash-Cipher (exercism)

;;; Commentary:

;;; Code:

(require 'seq)
(require 'subr-x)

(defun encodable-char-p (c)
  (or
   (and (>= c ?a) (<= c ?z))
   (and (>= c ?0) (<= c ?9))))

(defun encode-char (c)
  (cond ((and (>= c ?a) (<= c ?z))
         (- ?z (- c ?a)))
        ((and (>= c ?0) (<= c ?9))
         c)))

(defun encode (plaintext)
  "Encode PLAINTEXT to atbash-cipher encoding."
  (let ((encoded (seq-map #'encode-char
                          (seq-filter #'encodable-char-p (downcase plaintext)))))
    (string-join (seq-partition (apply #'string encoded) 5) " ")))

(provide 'atbash-cipher)
;;; atbash-cipher.el ends here
