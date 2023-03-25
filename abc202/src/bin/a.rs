#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: usize, b: usize, c: usize}

    println!("{}", 21 - a - b - c);
}
