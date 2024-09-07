#! /bin/sh
rm -rf report.json
cargo test -- -Z unstable-options --report-time --format json >> report.json

markdown-test-report -o index.md report.json