#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {a: i32, b: i32, c: i32, d: i32}
    let mut res = std::i32::MIN;
    for x in a..=b {
        for y in c..=d {
            res = std::cmp::max(res, x-y);
        }
    }
    println!("{}", res);
}
