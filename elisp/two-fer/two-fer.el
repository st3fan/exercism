;;; two-fer.el --- Two-fer Exercise (exercism)

;;; Commentary:

;;; Code:

(defun two-fer (&optional name)
  "Return a two-fer message"
  (format "One for %s, one for me." (or name "you")))

(provide 'two-fer)
;;; two-fer.el ends here
