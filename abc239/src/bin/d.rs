#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}
    if (a..=b).any(|k| (c..=d).all(|l| (2..(k+l)).any(|m| (k+l) % m == 0))) {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}
