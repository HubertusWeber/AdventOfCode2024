(ns d04 (:require [clojure.string :as str]))

(def grid (str/split-lines (slurp "input.txt")))
(def dirs (for [x [-1 0 1] y [-1 0 1] :when (not= 0 (+ (abs x) (abs y)))] [x y]))
(def h (count grid))
(def w (count (first grid)))

(defn at-pos [g r c [dx dy] n]
  (->> (range n)
       (map #(get-in g [(+ r (* % dx)) (+ c (* % dy))]))
       (#(when (every? some? %) (apply str %)))))

(def count-matches (comp count (partial filter identity)))

(defn find-word [word]
  (->> (for [r (range h)
             c (range w)
             d dirs
             :when (= word (at-pos grid r c d (count word)))] 1)
       count-matches))

(defn pat-matches [r c]
  (when (and (< (+ r 2) h) (< (+ c 2) w))
    (let [sub (for [r (range r (+ r 3))] (subs (grid r) c (+ c 3)))
          match? (fn [p c] (case p \M (= c \M) \S (= c \S) \A (= c \A) \? true))]
      (some #(every? (fn [[p g]] (every? true? (map match? p g))) (map vector % sub))
            [["M?S" "?A?" "M?S"] ["S?S" "?A?" "M?M"]
             ["M?M" "?A?" "S?S"] ["S?M" "?A?" "S?M"]]))))

(prn {:part1 (find-word "XMAS")
      :part2 (->> (for [r (range h)
                        c (range w)
                        :when (pat-matches r c)] 1)
                  count-matches)})
