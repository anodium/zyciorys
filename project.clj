(defproject zyciorys "0.1.0-SNAPSHOT"
  :description "Dynamic résumé renderer. Written because I hate typesetting."
  :url "https://github.com/anodium/zyciorys"
  :dependencies [[org.clojure/clojure "1.9.0"]
                 [org.clojure/clojurescript "1.10.339"]
                 [io.forward/yaml "1.0.9"]]
  :jvm-opts ^:replace ["-Xmx1g" "-server"]
  :plugins [[lein-npm "0.6.2"]]
  :npm {:dependencies [[source-map-support "0.4.0"]]}
  :source-paths ["src" "target/classes"]
  :clean-targets [:target-path "out" "release"]
  :target-path "target")
