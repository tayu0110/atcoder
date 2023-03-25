#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: i32, l: i32, r: i32}

    if s < l {
        println!("{}", l);
    } else if r < s {
        println!("{}", r);
    } else {
        println!("{}", s);
    }
}
