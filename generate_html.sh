#! /bin/bash

# apk add git

git config --global user.name "subhankar"
git config --global user.email "sub.nitt@gmail.com"

export directory="./contract-test-report"

if [ -d "$directory" ];then
    cd "$directory"
    git pull origin main
    cd ..
else
    git clone https://github.com/subhankar-nitt/contract-test-report.git

fi
pip install -r requirements.txt

python3 main.py

cd contract-test-report

git add .
git commit -m "added files"

# git pull origin main


git push origin main