#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: usize, a: [usize; n]}

    println!("{}", a.into_iter().filter(|c| *c < p).count());
}
