(ns zyciorys.core
  (:require
   [reagent.core :as react]
   [cljsjs.js-yaml :as yaml])) ; FIXME: Package not resolving

;; -------------------------
;; Views

(defn home-page [resume]
  [:div [:h2 resume]])

;; -------------------------
;; Initialize app
(def ^:dynamic *resume*
  (.. (.. js/window (fetch
                     (.. js/document (getElementById "resume")
                         (getAttribute "href"))))
      then (fn [declare response] ; FIXME: Variable not binding
             ((println response)
             (if (.. response Ok)
               (println "I'm OKAY!")
               (println "... Houston?"))))))

(defn mount-root []
  (react/render [home-page] (.. js/document (getElementById "app"))))

(defn init! []
  (mount-root))
