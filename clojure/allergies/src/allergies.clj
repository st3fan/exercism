(ns allergies)

(def *allergies*
  {:eggs 0
   :peanuts 1
   :shellfish 2
   :strawberries 3
   :tomatoes 4
   :chocolate 5
   :pollen 6
   :cats 7})

(defn allergic-to? [score allergy]
  (bit-test score (*allergies* allergy)))

(defn allergies [score]
  (filter #(allergic-to? score %) (keys *allergies*)))
