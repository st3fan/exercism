(ns meetup
  (:require [java-time :as t]))

(def teenths [13 14 16 17 18 19])

(defn find-teenth [month year day-name]
  (let [dow (t/day-of-week day-name)]
    (first (filter #(= (t/day-of-week (t/local-date year month %)) dow) teenths))))

(defn meetup [month year day-name when]
  (let [date (t/local-date year month) dow (t/day-of-week day-name)]
    (t/as
     (case when
       :first  (t/adjust date :first-in-month dow)
       :second (t/plus (t/adjust date :first-in-month dow) (t/weeks 1))
       :third  (t/plus (t/adjust date :first-in-month dow) (t/weeks 2))
       :fourth (t/plus (t/adjust date :first-in-month dow) (t/weeks 3))
       :last   (t/adjust date :last-in-month dow)
       :teenth (t/plus date (t/days (dec (find-teenth month year day-name)))))
     :year :month-of-year :day-of-month)))
