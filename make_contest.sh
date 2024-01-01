#!/bin/bash -l

if [ -z "$1" ]; then
	echo "Usage: ./make_contest.sh [CONTEST_NAME]"
	exit
fi

DIR="$1"

if ls "$DIR" >/dev/null 2>&1; then
	echo "$DIR already exists."
	exit
fi

cargo member new "$DIR"
cd "$DIR" || exit

mkdir .vscode -p
cat <<EOF >.vscode/settings.json
{
    "rust-analyzer.check.command": "check"
}
EOF

cat <<EOF >>Cargo.toml
ac-library-rs.workspace = true
convolution.workspace = true
ds.workspace = true
fenwick-tree.workspace = true
flow.workspace = true
geometry.workspace = true
graph.workspace = true
itertools.workspace = true
math.workspace = true
mincost-flow.workspace = true
montgomery-modint.workspace = true
num.workspace = true
ordered-float.workspace = true
permutohedron.workspace = true
polynomial.workspace = true
proconio.workspace = true
rand.workspace = true
rational.workspace = true
regex.workspace = true
rustc-hash.workspace = true
segtree.workspace = true
static-modint.workspace = true
string.workspace = true
suffix-array.workspace = true
superslice.workspace = true
unionfind.workspace = true
utility.workspace = true
EOF

mkdir -p src/bin
for prefix in {a..f}; do
	SRC=src/bin/$prefix.rs
	{
		printf "use proconio::*;\n\n"
		printf "fn main() {\n"
		printf "    \n"
		printf "}\n"
	} >>"$SRC"
	cargo equip --exclude-atcoder-crates --exclude-atcoder-202301-crates --minify libs --no-rustfmt --no-check --remove docs --remove comments --bin "$prefix" >/dev/null
done

cargo build
cargo build --release

code .
