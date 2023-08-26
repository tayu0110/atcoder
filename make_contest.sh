#!/bin/bash

if [ -z "$1" ]; then
	echo "Usage: ./make_contest.sh [CONTEST_NAME]"
	exit
fi

DIR="$1"

ls $DIR > /dev/null 2>&1

if [ $? -eq 0 ]; then
	echo "$DIR already exists."
	exit
fi

cargo new $DIR
cd $DIR

mkdir -p src/bin
for prefix in {a..f}
do
	SRC=src/bin/$prefix.rs
	printf "use proconio::*;\n\n" >> $SRC
	printf "fn main() {\n" >> $SRC
	printf "    \n" >> $SRC
	printf "}\n" >> $SRC
done

cargo add ac-library-rs@=0.1.1
cargo add num@=0.4.1
cargo add rand@=0.8.5
cargo add regex@=1.9.1
cargo add permutohedron@=0.2.4
cargo add superslice@=1.0.0
cargo add itertools@=0.11.0
cargo add proconio@=0.4.5 --features derive

cargo add --path ../../tayu-procon/convolution-simd
cargo add --path ../../tayu-procon/fenwick-tree/
cargo add --path ../../tayu-procon/flow
cargo add --path ../../tayu-procon/geometry
cargo add --path ../../tayu-procon/math
cargo add --path ../../tayu-procon/mincost-flow
cargo add --path ../../tayu-procon/modint/static-modint
cargo add --path ../../tayu-procon/modint/montgomery-modint
cargo add --path ../../tayu-procon/polynomial
cargo add --path ../../tayu-procon/rational
cargo add --path ../../tayu-procon/segtree
cargo add --path ../../tayu-procon/string
cargo add --path ../../tayu-procon/suffix-array
cargo add --path ../../tayu-procon/unionfind

cargo build
cargo build --release

code .
