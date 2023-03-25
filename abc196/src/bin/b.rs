#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: String}
    let v = x.split('.').collect::<Vec<_>>();
    println!("{}", v[0]);
}
