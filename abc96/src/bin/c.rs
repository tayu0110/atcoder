#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                continue;
            }

            if i + 1 < h && s[i + 1][j] == '#' {
                continue;
            }
            if j + 1 < w && s[i][j + 1] == '#' {
                continue;
            }
            if i > 0 && s[i - 1][j] == '#' {
                continue;
            }
            if j > 0 && s[i][j - 1] == '#' {
                continue;
            }

            println!("No");
            return;
        }
    }

    println!("Yes")
}
