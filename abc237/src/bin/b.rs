#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: usize, w: usize, mut a: [[usize; w]; h]}

    let mut b = vec![vec![0; h]; w];
    for i in 0..w {
        for j in 0..h {
            b[i][j] = a[j][i];
        }
    }

    for v in b {
        println!("{}", v.iter().join(" "));
    }
}
