#!/bin/bash

cd "./lune-src"
cargo build --release
cd ..

rm -f "lune"

ln -s "./lune-src/target/release/lune" "lune"

"./lune" setup
