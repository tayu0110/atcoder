#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut p: [(String, usize); n]}
    p.sort_by_key(|(_, t)| std::cmp::Reverse(*t));

    println!("{}", p[1].0);
}
