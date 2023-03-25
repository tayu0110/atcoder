#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: usize, w: usize, c: [Chars; h]};
    let mut x = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                x[j] += 1;
            }
        }
    }

    println!("{}", x.iter().join(" "));
}
