#!/bin/bash
NUMBER="1.1.0"
roc_PATH="/home/rafal"

cd "$roc_PATH"
roc_PATH="$roc_PATH/czkawka"
rm -rf $roc_PATH
git clone https://github.com/wx-chevalier/rust-os-cleaner.git "$roc_PATH"
cd $roc_PATH
git checkout "$NUMBER"

cd "$roc_PATH/roc_core"
cargo package
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed CORE"
  exit 1
fi
git reset --hard

cd "$roc_PATH/roc_core"
cargo publish
git reset --hard

