;;; word-count.el --- word-count Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)
(require 'subr-x)

(defun increment-key (table key)
  (puthash key (1+ (gethash key table 0)) table))

(defun hash-table-data (table)
  "Returns the contents of TABLE as a list"
  (let ((data nil))
    (maphash (lambda (k v) (setq data (cons (cons k v) data))) table)
    data))

(defun word-count (phrase)
  (let ((counts (make-hash-table :test 'equal)))
    (thread-last (split-string phrase "[^a-z0-9]" t) ;;  "\\W")
      (seq-map #'downcase)
      ;;(seq-filter (lambda (s) (not (zerop (length s)))))
      (seq-do (apply-partially #'increment-key counts)))
    (hash-table-data counts)))

(word-count "car : carpet as java : javascript!!&@$%^&")

(provide 'word-count)
;;; word-count.el ends here
