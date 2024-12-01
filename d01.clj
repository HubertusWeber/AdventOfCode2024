(ns d01
  (:require
   [clojure.java.io :as io]
   [clojure.string :as str]))

(defn parse-numbers [filename]
  (with-open [rdr (io/reader filename)]
    (->> (line-seq rdr)
         (mapv #(mapv parse-long (str/split (str/trim %) #"\s+")))
         (apply map vector)
         (mapv vec))))

(let [[left right] (parse-numbers "input.txt")]
  {:part1 (->> (map - (sort left) (sort right))
               (map abs)
               (reduce +))
   :part2 (->> (frequencies right)
               (#(map (fn [n] (* n (get % n 0))) left))
               (reduce +))})
