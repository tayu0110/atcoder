#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, mut x: usize, m: [usize; n]}

    x -= m.iter().sum::<usize>();

    println!("{}", n + x / m.into_iter().min().unwrap());
}
