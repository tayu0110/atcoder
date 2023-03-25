#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {a: i32, b: i32}

    if a > b {
        println!("0");
    } else {
        println!("{}", b - a + 1);
    }
}
