#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {_n: usize, s: Chars}

    let mut res = 0;
    for v in s.windows(3) {
        if v.iter().collect::<String>() == "ABC" {
            res += 1;
        }
    }

    println!("{}", res);
}
