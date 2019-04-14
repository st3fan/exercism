;;; pangram.el --- Pangram (exercism)

(require 'subr-x)
(require 'seq)

;;; Commentary:

;;; Code:

(defun is-pangram (s)
  (thread-last s
    (downcase)
    (string-to-list)
    (seq-filter (lambda (c) (and (>= c ?a) (<= c ?z))))
    (seq-uniq)
    (seq-length)
    (= 26)))

(provide 'pangram)
;;; pangram.el ends here
