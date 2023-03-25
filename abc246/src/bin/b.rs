#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {a: usize, b: usize}

    let d = ((a * a + b * b) as f64).sqrt();
    println!("{} {}", a as f64 / d, b as f64 / d);
}
