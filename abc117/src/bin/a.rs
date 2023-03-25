#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {t: f64, x: f64}

    println!("{:.10}", t / x)
}
