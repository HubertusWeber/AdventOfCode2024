(ns d05
  (:require [clojure.string :as str]))

(def input-text (slurp "input.txt"))

(def rules
  (->> (str/split input-text #"\n\n")
       first
       str/split-lines
       (mapv (fn [line]
               (->> (str/split line #"\|")
                    ((fn [[b a]]
                       {:before (Integer/parseInt b)
                        :after (Integer/parseInt a)})))))))

(def updates
  (->> (str/split input-text #"\n\n")
       second
       str/split-lines
       (mapv (comp #(mapv (fn [n] (Integer/parseInt n)) %)
                   #(str/split % #",")))))

(defn ordered? [rule nums]
  (->> (map-indexed #(vector %2 %1) nums)
       (into {})
       ((fn [idxs]
          (or (nil? (idxs (:before rule)))
              (nil? (idxs (:after rule)))
              (< (idxs (:before rule))
                 (idxs (:after rule))))))))

(def filter-updates
  (partial filter #(every? (fn [r] (ordered? r %)) rules)))

(def middle-element #(nth % (quot (count %) 2)))

(defn get-dependencies [rules]
  (reduce (fn [acc {:keys [before after]}]
            (update acc before (fnil conj #{}) after))
          {}
          rules))

(defn topological-sort [deps elems]
  ((fn step [res rem seen]
     (if (empty? rem)
       res
       (if-let [next (->> rem
                          (filter #(every? (fn [e]
                                             (or (seen e)
                                                 (not ((get deps % #{}) e))))
                                           rem))
                          first)]
         (recur (conj res next)
                (disj rem next)
                (conj seen next))
         res)))
   [] elems #{}))

(def reorder-update
  (comp vec #(topological-sort (get-dependencies rules) (set %))))

(def valid-updates (filter-updates updates))
(def invalid-updates (remove (set valid-updates) updates))

(prn {:part1 (->> valid-updates
                  (map middle-element)
                  (reduce +))
      :part2 (->> invalid-updates
                  (map reorder-update)
                  (map middle-element)
                  (reduce +))})
