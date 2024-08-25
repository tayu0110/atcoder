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

cargo new "$DIR"
cd "$DIR" || exit

mkdir .vscode -p
cat <<EOF >.vscode/settings.json
{
    "rust-analyzer.check.command": "check",
    "rust-analyzer.check.overrideCommand": [
        "cargo",
        "check",
        "--workspace",
        "--message-format=json",
        "--all-targets",
    ],
    "rust-analyzer.cargo.buildScripts.overrideCommand": [
        "cargo",
        "check",
        "--quiet",
        "--workspace",
        "--message-format=json",
        "--all-targets",
    ],
}
EOF

mkdir .cargo -p
cat <<EOF >.cargo/config.toml
[build]
target-dir = "../target"
EOF

echo "1.70.0" >> rust-toolchain

cargo add "ac-library-rs@=0.1.1"
cargo add num@=0.4.1
cargo add rand@=0.8.5
cargo add regex@=1.9.1
cargo add permutohedron@=0.2.4
cargo add superslice@=1.0.0
cargo add "rustc-hash@=1.1.0"
cargo add itertools@=0.11.0
cargo add proconio@=0.4.5 --features derive

cargo add bitset --git https://github.com/tayu0110/tayu-procon.git
cargo add complex --git https://github.com/tayu0110/tayu-procon.git
cargo add convolution --git https://github.com/tayu0110/tayu-procon.git
cargo add cpgraph --git https://github.com/tayu0110/tayu-procon.git
cargo add fenwick-tree --git https://github.com/tayu0110/tayu-procon.git
cargo add flow --git https://github.com/tayu0110/tayu-procon.git
cargo add geometry --git https://github.com/tayu0110/tayu-procon.git
cargo add graph --git https://github.com/tayu0110/tayu-procon.git
cargo add math --git https://github.com/tayu0110/tayu-procon.git
cargo add matrix --git https://github.com/tayu0110/tayu-procon.git
cargo add mincost-flow --git https://github.com/tayu0110/tayu-procon.git
cargo add montgomery-modint --git https://github.com/tayu0110/tayu-procon.git
cargo add polynomial --git https://github.com/tayu0110/tayu-procon.git
cargo add rational --git https://github.com/tayu0110/tayu-procon.git
cargo add segtree --git https://github.com/tayu0110/tayu-procon.git
cargo add static-modint --git https://github.com/tayu0110/tayu-procon.git
cargo add string --git https://github.com/tayu0110/tayu-procon.git
cargo add unionfind --git https://github.com/tayu0110/tayu-procon.git
cargo add utility --git https://github.com/tayu0110/tayu-procon.git
echo 'ds = { git = "https://github.com/tayu0110/tayu-procon.git", version = "0.1.0", features = ["full",] }' >> Cargo.toml

mkdir -p src/bin
for prefix in {a..g}; do
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
