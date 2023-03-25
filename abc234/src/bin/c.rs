#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {k: usize}

    println!("{}", format!("{:b}", k).replace('1', "2"));
}
