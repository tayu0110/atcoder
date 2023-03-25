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

cargo add proconio@0.3.8 --features derive
cargo add itertools@0.9.0
cargo add num@0.2.1
cargo add rand@0.7.3
cargo add permutohedron@0.2.4

cargo add convolution-simd --path ../../tayu-procon/convolution-simd
cargo add flow --path ../../tayu-procon/flow
cargo add geometry --path ../../tayu-procon/geometry
cargo add math --path ../../tayu-procon/math
cargo add mincost-flow --path ../../tayu-procon/mincost-flow
cargo add --path ../../tayu-procon/modint/static-modint
cargo add --path ../../tayu-procon/modint/montgomery-modint
cargo add polynomial --path ../../tayu-procon/polynomial
cargo add rational --path ../../tayu-procon/rational
cargo add segtree --path ../../tayu-procon/segtree
cargo add string --path ../../tayu-procon/string
cargo add suffix-array --path ../../tayu-procon/suffix-array

cargo build
cargo build --release

code .
