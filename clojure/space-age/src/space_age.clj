(ns space-age)

(def earth-years-per-planet
  {:earth   1.0
   :mercury 0.2408467
   :venus   0.61519726
   :mars    1.8808158
   :jupiter 11.862615
   :saturn  29.447498
   :uranus  84.016846
   :neptune 164.79132})

(defn earth-seconds-on-planet [seconds planet]
  {:pre [(contains? earth-years-per-planet planet)]}
  (/ seconds (* 31557600 (earth-years-per-planet planet))))

(defn on-mercury [seconds]
  (earth-seconds-on-planet seconds :mercury))

(defn on-venus [seconds]
  (earth-seconds-on-planet seconds :venus))

(defn on-earth [seconds]
  (earth-seconds-on-planet seconds :earth))

(defn on-mars [seconds]
  (earth-seconds-on-planet seconds :mars))

(defn on-jupiter [seconds]
  (earth-seconds-on-planet seconds :jupiter))

(defn on-saturn [seconds]
  (earth-seconds-on-planet seconds :saturn))

(defn on-neptune [seconds]
  (earth-seconds-on-planet seconds :neptune))

(defn on-uranus [seconds]
  (earth-seconds-on-planet seconds :uranus))
