#! /bin/sh
rm -rf report.json
cargo install markdown-test-report
cargo test -- -Z unstable-options --report-time --format json >> report.json
git config --global user.name "subhankar"
git config --global user.emanil "biswas"
mkdir contract-testing
cd contract-testing
git init
git remote add origin https://github.com/subhankar-nitt/rust-contract-testing.git
git pull
git checkout test_id

cd ..
markdown-test-report -o contract-testing/index.md report.json

cd contract-testing
git add index.md
git commit -m "commited file"
git push origin test_id