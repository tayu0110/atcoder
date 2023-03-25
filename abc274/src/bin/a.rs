#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {a: usize, b: usize};

    println!("{:.3}", b as f64 / a as f64);
}
