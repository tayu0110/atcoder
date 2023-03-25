#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {x: i64}

    if x >= 0 {
        println!("{}", x / 10);
    } else {
        if x % 10 == 0 {
            println!("{}", x / 10);
        } else {
            println!("{}", x / 10 - 1);
        }
    }
}
