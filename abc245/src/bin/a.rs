#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    if a < c {
        println!("Takahashi");
    } else if a > c {
        println!("Aoki");
    } else {
        if b <= d {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
