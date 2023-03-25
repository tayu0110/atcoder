#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {k: usize, x: usize}

    if 500 * k >= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
