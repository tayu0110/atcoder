#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: [(usize, char); n]}

    let mut red = vec![];
    let mut blue = vec![];
    for (x, c) in p {
        if c == 'R' {
            red.push(x);
        } else {
            blue.push(x);
        }
    }

    red.sort();
    blue.sort();

    for r in red {
        println!("{}", r);
    }
    for b in blue {
        println!("{}", b);
    }
}
