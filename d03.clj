(ns d03)

(defn sum-muls [s]
  (->> s
       (re-seq #"mul\((\d+),(\d+)\)")
       (map #(* (parse-long (nth % 1))
                (parse-long (nth % 2))))
       (reduce + 0)))

(defn do-or-dont [s]
  (loop [s s, res "", deleting? false]
    (cond
      (empty? s) res
      (and (not deleting?) (.startsWith s "don't()")) (recur (subs s 7) res true)
      (and deleting? (.startsWith s "do()")) (recur (subs s 4) res false)
      (not deleting?) (recur (subs s 1) (str res (first s)) false)
      :else (recur (subs s 1) res true))))

(prn {:part1 (-> "input.txt" slurp sum-muls)
      :part2 (-> "input.txt" slurp do-or-dont sum-muls)})
