(ns gigasecond
  (:import java.util.Calendar)
  (:import (java.time LocalDateTime Duration))
  (:require [java-time :as jt]))

;; Using java.util.Calendar - It works.

(defn from [year month day]
  (let [cal (Calendar/getInstance)]
    (doto cal
      (.set year (dec month) day)
      (.add Calendar/SECOND 1000000000))
    [(.get cal Calendar/YEAR)
     (inc (.get cal Calendar/MONTH))
     (.get cal Calendar/DAY_OF_MONTH)]))

;; Using java.time - Much nicer.

(defn from [year month day]
  (let [start (LocalDateTime/of year month day 0 0 0)
        final (.plus start (Duration/ofSeconds 1000000000))]
    [(.getYear final) (.getMonthValue final) (.getDayOfMonth final)]))

;; Using clojure.java-time - Clojure!

(defn from [year month day]
  (let [start (jt/local-date-time year month day)
        final (jt/plus start (jt/seconds 1000000000))]
    (jt/as final :year :month-of-year :day-of-month)))
