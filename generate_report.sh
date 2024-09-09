#! /bin/sh
rm -rf report.json
cargo build 
cargo install junitify
cargo test -- -Z unstable-options --report-time --format=json | junitify -i --out reports/
# markdown-test-report -o index.md report.json

# cat index.md

