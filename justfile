build:
  cargo build --profile=release-with-debug

record-miss-branches: build
  perf record -e 'branches:ppp,branch-misses:ppp' -g ./target/release-with-debug/miss_branches

record-hit-branches: build
  perf record -e 'branches:ppp,branch-misses:ppp' -g ./target/release-with-debug/hit_branches

report:
  perf report -g -M intel
