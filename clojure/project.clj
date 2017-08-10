(defproject project-euler "0.1.0-SNAPSHOT"
  :description "FIXME: write description"
  :url "http://example.com/FIXME"
  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}
  :dependencies [[org.clojure/clojure "1.8.0"]]
  :main ^:skip-aot project-euler.core
  :target-path "target/%s"
  :profiles {
    :problem-001 {:main problem-001.core}
    :problem-002 {:main problem-002.core}
    :problem-003 {:main problem-003.core}
    :problem-004 {:main problem-004.core}
  })
