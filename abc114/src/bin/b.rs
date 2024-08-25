#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {s: String}

    let len = s.len();
    let mut res = std::usize::MAX;
    for i in 0..=len-3 {
        let k = s[i..i+3].parse().unwrap();
        let diff = std::cmp::max(753usize, k) - std::cmp::min(753usize, k);
        res = std::cmp::min(res, diff);
    }

    println!("{}", res);
}
