(ns triangle)

(defn is-valid? [a b c]
  (and (> a 0)
       (> b 0)
       (> c 0)
       (>= (+ a b) c)
       (>= (+ b c) a)
       (>= (+ c a) b)))

(defn equilateral? [a b c]
  (and (is-valid? a b c)
       (= a b c)))

(defn isosceles? [a b c]
  (and (is-valid? a b c)
       (or (= a b)
           (= b c)
           (= a c))))

(defn scalene? [a b c]
  (and (is-valid? a b c)
       (not= a b)
       (not= a c)
       (not= b c)))
