#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let mut max = 0;
    for &(x, y) in &p {
        for &(nx, ny) in &p {
            max = std::cmp::max(max, (x-nx) * (x-nx) + (y-ny) * (y-ny));
        }
    }

    println!("{}", (max as f64).sqrt());
}
