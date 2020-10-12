#!/bin/bash
NUMBER="1.1.0"
roc_PATH="/home/rafal"

cd "$roc_PATH"
roc_PATH="$roc_PATH/czkawka"
rm -rf $roc_PATH
git clone https://github.com/wx-chevalier/rust-os-cleaner.git "$roc_PATH"
cd $roc_PATH
git checkout "$NUMBER"


cd "$roc_PATH/roc_cli"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_cli/Cargo.toml"
cargo package --allow-dirty
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed CLI"
  exit 1
fi
git reset --hard


cd "$roc_PATH/roc_gui"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_gui/Cargo.toml"
cargo package --allow-dirty
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed GUI"
  exit 1
fi
git reset --hard

cd "$roc_PATH/roc_gui_orbtk"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_gui_orbtk/Cargo.toml"
cargo package --allow-dirty
if [ $(echo $?) != "0"  ]
then
  echo "Cargo package failed GUI ORBTK"
  exit 1
fi
git reset --hard



cd "$roc_PATH/roc_cli"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_cli/Cargo.toml"
cargo publish --allow-dirty
git reset --hard

cd "$roc_PATH/roc_gui"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_gui/Cargo.toml"
cargo publish --allow-dirty
git reset --hard

cd "$roc_PATH/roc_gui_orbtk"
sed -i "s/{ path = \"..\/roc_core\" }/\"=$NUMBER\"/g" "$roc_PATH/roc_gui_orbtk/Cargo.toml"
cargo publish --allow-dirty
git reset --hard
