#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {d: usize, t: usize, s: usize}

    if s * t >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
