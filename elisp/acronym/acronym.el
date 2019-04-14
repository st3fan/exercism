;;; acronym.el --- Acronym (exercism)

;;; Commentary:

;;; Code:

(defun acronym (phrase)
  (upcase
   (apply #'concat (mapcar
                    (lambda (s)
                      (substring s 0 1))
                    (split-string phrase "\\W" t)))))

(provide 'acronym)
;;; acronym.el ends here
