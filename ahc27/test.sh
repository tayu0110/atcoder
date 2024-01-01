#!/bin/bash

rm tools/out -rf
mkdir tools/out -p

cargo build 2>/dev/null
cargo build --release 2>/dev/null

cd tools/ || target/release/gen seeds.txt
cd ../

seq 0 500 |
  xargs -I@ printf "target/release/ahc27 < tools/in/0%03d.txt > tools/out/0%03d.txt\n" @ @ |
  # xargs -I@ printf "./tools/target/release/tester ./target/release/ahc26 < ./tools/in/%04d.txt > ./tools/out/%04d.txt\n" @ @ |
  xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @
seq 0 500 |
  xargs -I@ printf "tools/target/release/vis tools/in/%04d.txt tools/out/%04d.txt\n" @ @ |
  # xargs -L1 -n1 -I@ bash -c @ #|
  xargs -L1 -n1 -P"$(nproc)" -I@ bash -c @ |
  sed -r 's/Score = ([0-9]+)/\1/' |
  awk 'BEGIN{a = 0}{a += $0}END{print a}'
