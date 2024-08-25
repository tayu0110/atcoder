#!/bin/bash

rm tools/in -rf
mkdir tools/in -p

rm tools/out -rf
mkdir tools/out -p

cargo build 2>/dev/null
cargo build --release 2>/dev/null

SEED_MIN=0
SEED_MAX=500

seq $SEED_MIN $SEED_MAX >tools/seeds.txt

cd tools/ || exit
cargo run --release --bin gen seeds.txt
cd ../

seq $SEED_MIN $SEED_MAX |
  # xargs -I@ printf "tools/target/release/tester target/release/ahc30 < tools/in/%04d.txt > tools/out/%04d.txt 2> /dev/null\n" @ @ |
  xargs -I@ printf "../target/release/ahc31 < tools/in/%04d.txt > tools/out/%04d.txt\n" @ @ |
  # xargs -L1 -n1 -I@ bash -c @
  xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @
# xargs -L1 -n1 -I@ bash -c @ 2>&1 | grep "millis" | sort | uniq
# xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @ 2>&1 | grep "millis" | sort | uniq
seq $SEED_MIN $SEED_MAX |
  xargs -I@ printf "../target/release/vis tools/in/%04d.txt tools/out/%04d.txt\n" @ @ |
  # xargs -L1 -n1 -I@ bash -c @
  xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @ |
  sed -r 's/Score = ([0-9]+)/\1/' |
  awk 'BEGIN{a = 0}{a += $0}END{print a}'
