;;; allergies.el --- Allergies Exercise (exercism)

(require 'subr-x)

;;; Commentary:

;;; Code:

(defconst allergens
  '(("eggs" . 1)
    ("peanuts" . 2)
    ("shellfish" . 4)
    ("strawberries" . 8)
    ("tomatoes" . 16)
    ("chocolate" . 32)
    ("pollen" . 64)
    ("cats" . 128)))

(defun allergen-list (score)
  (mapcan
   (lambda (a)
     (if (eq (logand score (cdr a)) (cdr a))
         (list (car a))))
   allergens))

(defun allergic-to-p (score thing)
  (when-let ((a (assoc thing allergens)))
    (eq (logand score (cdr a)) (cdr a))))

(provide 'allergies)
;;; allergies.el ends here
