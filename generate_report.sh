#! /bin/sh
rm -rf report.json
cargo install markdown-test-report
cargo test -- -Z unstable-options --report-time --format json >> report.json
markdown-test-report -o index.md report.json

cat index.md

