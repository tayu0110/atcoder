#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize}

    println!("{}", (1..n).map(|v| n as f64 / v as f64).sum::<f64>());
}
