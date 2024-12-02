(ns d02
  (:require
   [clojure.string :as str]))

(def input
  (->> (slurp "input.txt")
       str/split-lines
       (mapv #(mapv parse-long (str/split (str/trim %) #"[,\s]+")))))

(defn safe? [nums]
  (let [diffs (map - (rest nums) nums)]
    (and (or (every? pos? diffs)
             (every? neg? diffs))
         (every? #(<= -3 % 3) diffs))))

(defn tolerantly-safe? [nums]
  (or (safe? nums)
      (some #(safe? (concat (take % nums) (drop (inc %) nums)))
            (range (count nums)))))

{:part1 (->> input (filter safe?) count)
 :part2 (->> input (filter tolerantly-safe?) count)}
