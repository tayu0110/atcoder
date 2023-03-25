#![allow(unused_imports)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {h: usize, w: usize, mut s: [[usize; w]; h], n: usize, p: [(usize, usize); n]}

    for (r, c) in p {
        s[r - 1][c - 1] = 0;

        for i in (0..h).rev() {
            for j in 0..w {
                if s[i][j] == 0 && i > 0 {
                    s[i][j] = s[i - 1][j];
                    s[i - 1][j] = 0;
                }
            }
        }
    }

    for i in 0..h {
        println!("{}", s[i].iter().join(" "));
    }
}
