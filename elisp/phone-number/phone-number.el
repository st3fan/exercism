;;; phone-number.el --- phone-number Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)
(require 'subr-x)

;; Are regexps horrible in emacs or is there a better less quote'ey way?
(defconst phone-number-formats
  '("^1\\([0-9]\\{10\\}\\)$"	  	                                ; 11234567890
    "^\\([0-9]\\{10\\}\\)$"	  	                                ; 1234567890
    "^\\([0-9]\\{3\\}\\)-\\([0-9]\\{3\\}\\)-\\([0-9+]\\{4\\}\\)$"       ; 123-456-7890
    "^(\\([0-9]\\{3\\}\\)) \\([0-9]\\{3\\}\\)-\\([0-9+]\\{4\\}\\)$"     ; (123) 456-7890
    "^\\([0-9]\\{3\\}\\)\\.\\([0-9]\\{3\\}\\)\\.\\([0-9+]\\{4\\}\\)$")) ; 123.456.7890

(defun phone-number-match (number format)
  "Match NUMBER against the FORMAT regexp."
  (string-match format number))

(defun numbers (phone-number)
  "Parse PHONE-NUMBER and return the normalized form."
  (if (seq-find (apply-partially 'phone-number-match phone-number) phone-number-formats)
      (concat
       (match-string 1 phone-number)
       (match-string 2 phone-number)
       (match-string 3 phone-number))
    "0000000000"))

(defun area-code (phone-number)
  "Return the area code of PHONE-NUMBER."
  (substring phone-number 0 3))

(defun pprint (phone-number)
  "Pretty print PHONE-NUMBER"
  (let ((phone-number (numbers phone-number)))
    (format "(%s) %s-%s"
            (substring phone-number 0 3)
            (substring phone-number 3 6)
            (substring phone-number 6 10))))

(provide 'phone-number)
;;; phone-number.el ends here
