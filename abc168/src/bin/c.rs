#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {a: i64, b: i64, h: i64, m: i64}

    let theta = (60 * h + m - 12 * m).abs() as f64 * std::f64::consts::PI / 360.0;
    let c2 = (a * a + b * b) as f64 - (2 * a * b) as f64 * theta.cos();

    println!("{:.20}", c2.sqrt());
}
