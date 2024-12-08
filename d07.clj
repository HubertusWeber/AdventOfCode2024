(ns d07
  (:require [clojure.string :as str]))

(defn parse-line [line]
  (let [[tv nums] (str/split line #":\s")]
    [(Long. tv) (mapv #(Long. %) (str/split nums #"\s+"))]))

(defn all-operator-combinations [n]
  (for [combo (range (bit-shift-left 1 (dec n)))]
    (mapv #(if (bit-test combo %) :* :+) (range (dec n)))))

(defn evaluate-sequence [nums ops]
  (reduce (fn [acc [op num]] (({:+ + :* *} op) acc num))
          (first nums)
          (map vector ops (rest nums))))

(defn equation-possible? [tv nums]
  (some #(= tv (evaluate-sequence nums %))
        (all-operator-combinations (count nums))))

(prn {:part1 (->> (slurp "input.txt")
                  str/split-lines
                  (map parse-line)
                  (filter (fn [[tv nums]] (equation-possible? tv nums)))
                  (map first)
                  (reduce +))})
