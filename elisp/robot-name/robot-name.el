;;; robot-name.el --- Robot Name (exercism)

;;; Commentary:
;;
;; Build a robot with a name like AA000, that can be reset
;; to a new name. Instructions are in README.md
;;

;;; Code:

(require 'cl-lib)

(defun random-robot-name ()
  (format
   "%c%c%d%d%d"
   (+ ?A (random 26))
   (+ ?A (random 26))
   (random 10)
   (random 10)
   (random 10)))

(cl-defstruct robot
  (name (random-robot-name)))

(defun build-robot ()
  (make-robot))

(defun reset-robot (robot)
  (setf (robot-name robot) (random-robot-name)))

(provide 'robot-name)
;;; robot-name.el ends here
