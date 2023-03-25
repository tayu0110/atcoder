#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, h: [usize; n]}
    let max = h.iter().max().unwrap();
    for (i, v) in h.iter().enumerate() {
        if v == max {
            println!("{}", i+1);
        }
    }
}
