#!/bin/bash

rm tools/in -rf
mkdir tools/in -p

rm tools/out -rf
mkdir tools/out -p

cargo build 2>/dev/null
cargo build --release 2>/dev/null

SEED_MIN=0
SEED_MAX=150

seq $SEED_MIN $SEED_MAX >tools/seeds.txt

cd tools/ || exit
cargo run --release --bin gen seeds.txt
cd ../

echo "Test Data is successfully generated!!"

seq $SEED_MIN $SEED_MAX |
  # xargs -I@ printf "tools/target/release/tester target/release/ahc30 < tools/in/%04d.txt > tools/out/%04d.txt 2> /dev/null\n" @ @ |
  xargs -I@ printf "../target/release/ahc32 < tools/in/%04d.txt > tools/out/%04d.txt\n" @ @ |
  xargs -L1 -n1 -I@ bash -c @
# xargs -I@ -L1 -n1 -P"$(nproc)" bash -c @

echo "All Test are finished."

seq $SEED_MIN $SEED_MAX |
  xargs -I@ printf "../target/release/vis tools/in/%04d.txt tools/out/%04d.txt\n" @ @ |
  # xargs -L1 -n1 -I@ bash -c @ |
  xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @ |
  sed -r 's/Score = ([0-9]+)/\1/' |
  awk 'BEGIN{a = 0}{a += $0}END{print a}'
