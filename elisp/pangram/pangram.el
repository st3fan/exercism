;;; pangram.el --- Pangram (exercism)

(require 'subr-x)
(require 'seq)

;;; Commentary:

;;; Code:

(defun is-pangram (s)
  (eq 26 (thread-last s
    (downcase)
    (string-to-list)
    (seq-filter (lambda (c) (and (>= c 97) (<= c 122))))
    (seq-uniq)
    (seq-length))))

(provide 'pangram)
;;; pangram.el ends here
