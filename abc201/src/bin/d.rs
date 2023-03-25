#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {h: usize, w: usize, a: [Chars; h]}

    let mut c = vec![vec![std::i32::MIN; w]; h];
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            if i == h - 1 && j == w - 1 {
                c[i][j] = 0;
                continue;
            }

            if j + 1 < w {
                c[i][j] = std::cmp::max(
                    c[i][j],
                    -c[i][j + 1] + if a[i][j + 1] == '+' { 1 } else { -1 },
                );
            }
            if i + 1 < h {
                c[i][j] = std::cmp::max(
                    c[i][j],
                    -c[i + 1][j] + if a[i + 1][j] == '+' { 1 } else { -1 },
                );
            }
        }
    }

    if c[0][0] < 0 {
        println!("Aoki");
    } else if c[0][0] == 0 {
        println!("Draw");
    } else {
        println!("Takahashi");
    }
}
