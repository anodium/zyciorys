(require '[cljs.build.api :as b])

(b/watch "src"
  {:main 'zyciorys.core
   :output-to "out/zyciorys.js"
   :output-dir "out"})
