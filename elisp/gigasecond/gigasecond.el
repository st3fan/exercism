;;; gigasecond.el --- Gigasecond exercise (exercism)

;;; Commentary:
;; Calculate the date one gigasecond (10^9 seconds) from the
;; given date.
;;
;; NB: Pay attention to  Emacs' handling of time zones and dst
;; in the encode-time and decode-time functions.

;;; Code:

(require 'parse-time)
(require 'seq)

(defun add-gigasecond (hour minute second day month year)
  "Add a gigasecond to the specified time"
  (decode-time
   (time-add
    (encode-time hour minute second day month year "UTC")
    (seconds-to-time 1000000000)) "UTC"))

(defun from (hour minute second day month year)
  (seq-take (add-gigasecond hour minute second day month year) 6))

(provide 'gigasecond)
;;; gigasecond.el ends here
