(ns zyciorys.prod
  (:require
    [zyciorys.core :as core]))

;;ignore println statements in prod
(set! *print-fn* (fn [& _]))

(core/init!)
