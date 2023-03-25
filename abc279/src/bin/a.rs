#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {s: Chars}
    let w = s.iter().filter(|c| **c == 'w').count();
    let v = s.into_iter().filter(|c| *c == 'v').count();

    println!("{}", w * 2 + v);
}
