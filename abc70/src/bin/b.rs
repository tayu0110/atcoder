#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: u8, b: u8, c: u8, d: u8}

    let v = (a..b).collect::<std::collections::HashSet<_>>();
    let w = (c..d).collect::<std::collections::HashSet<_>>();

    println!("{}", v.intersection(&w).count())
}
