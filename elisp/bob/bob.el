;;; bob.el --- Bob exercise (exercism)

;;; Commentary:

;;; Code:

(require 'subr-x)

(defun question-p (message)
  (string-suffix-p "?" message))

(defun yelling-p (message)
  (and
   (string-match "[a-z]" message)
   (equal (upcase message) message)))

(defun response-for (message)
  "Return Bob's response to MESSAGE."
  (let ((message (string-trim message)))
    (cond ((= 0 (length message)) "Fine. Be that way!")
          ((and (question-p message) (yelling-p message)) "Calm down, I know what I'm doing!")
          ((question-p message) "Sure.")
          ((yelling-p message) "Whoa, chill out!")
          (t "Whatever."))))

(provide 'bob)
;;; bob.el ends here
