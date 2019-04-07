;;; phone-number.el --- phone-number Exercise (exercism)

;;; Commentary:

;;; Code:

(require 'seq)

(defconst number-formats
  '("^1(\d{9})$"                        ; 1234567890
    "^+1 \((\d{3})\) (\d{3})-(\d{4})$"  ; +1 (123) 456-7890
    "^(\d{3})-(\d{3})-(\d{4})$"         ; 123-456-7890
    "^1 (\d{3}) (\d{3}) (\d{4})$"       ; 1 123 456 7890
    "^(\d{3}).(\d{3}).(\d{4})$"))       ; 123.456.7890

(defun match-number (number format)
  nil)

(string-match "^\\([0-9]{3}\\)-\\([0-9]{3}\\)-\\([0-9]{4}\\)$" "(123)-(456)-(7890)")

(progn
  (let ((s "123-456-7890"))
    (string-match "^\\([0-9]+\\)-\\([0-9]+\\)-\\([0-9+]+\\)$" s)
    (list (match-string 0 s) (match-string 1 s) (match-string 2 s) (match-string 3 s)))

;;

(defun numbers (number)
  (seq-find (apply-partially 'match-number number) number-formats))

(numbers "1234567890")

(provide 'phone-number)
;;; phone-number.el ends here
