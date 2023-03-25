#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: f64, b: f64}

    println!("{}", 100.0 - (b / a) * 100.0);
}
