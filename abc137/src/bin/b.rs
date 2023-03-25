#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {k: i32, x: i32}

    println!("{}", (x - k + 1..x + k).into_iter().join(" "));
}
