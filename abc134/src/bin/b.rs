#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, d: usize}
    let w = 2 * d + 1;

    println!("{}", (n + w - 1) / w);
}
