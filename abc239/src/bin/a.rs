#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {h: usize}

    println!("{}", (h as f64 * (12800000 + h) as f64).sqrt());
}
