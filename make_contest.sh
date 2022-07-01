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
	printf "#[allow(unused_imports)]\n" > $SRC
	printf "use proconio::{input, marker::Chars, source::line::LineSource};\n\n" >> $SRC
	printf "fn main() {\n" >> $SRC
	printf "\t\n" >> $SRC
	printf "}\n" >> $SRC
done

cargo add proconio
cargo add itertools itertools-num
cargo add num num-derive
cargo add ndarray nalgebra
cargo add rand rand_distr
cargo add petgraph indexmap lazy_static ordered-float ascii permutohedron maplit either im-rc fixedbitset text_io whiteread rustc-hash smallvec

cargo build

code .
