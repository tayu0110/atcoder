#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {a: f64, b: f64}
    println!("{}", (a + 2.0*b) / 3.0)
}
