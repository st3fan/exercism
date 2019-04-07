;;; anagram.el --- Anagram (exercism)

;;; Commentary:

;;; Code:

(require 'seq)

(defun sort-word (word)
  (concat (sort (string-to-list (upcase word)) '>)))

(defun anagrams-for (word possible-anagrams)
  (let ((sorted-word (sort-word word)))
    (seq-filter
     (lambda (possible)
       (and
        (not (string-equal word possible))
        (string-equal sorted-word (sort-word possible))))
     possible-anagrams)))

(provide 'anagram)
;;; anagram.el ends here
