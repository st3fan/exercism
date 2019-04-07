;;; raindrops.el --- Raindrops (exercism)

;;; Commentary:

;;; Code:

(require 'subr-x)

(defun convert (n)
  (let ((result (concat
                 (if (zerop (mod n 3)) "Pling")
                 (if (zerop (mod n 5)) "Plang")
                 (if (zerop (mod n 7)) "Plong"))))
    (if (string-empty-p result)
        (number-to-string n)
      result)))

(provide 'raindrops)
;;; raindrops.el ends here
