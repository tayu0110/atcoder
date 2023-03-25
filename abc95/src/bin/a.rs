#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: Chars}

    println!("{}", 700 + s.into_iter().filter(|c| *c == 'o').count() * 100);
}
