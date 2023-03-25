#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    println!("{}", std::cmp::max(a * b, c * d))
}
