#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, k: usize, mut s: [String; n]}

    s[..k].sort();

    println!("{}", s.iter().take(k).join("\n"))
}
