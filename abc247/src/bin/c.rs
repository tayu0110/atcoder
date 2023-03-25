#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn rec(n: usize, res: &mut Vec<usize>) {
    if n > 1 {
        rec(n-1, res);
    }
    res.push(n);
    if n > 1 {
        rec(n-1, res);
    }
}

#[fastout]
fn main() {
    input! {n: usize}

    let mut res = vec![];
    rec(n, &mut res);
    println!("{}", res.iter().join(" "));
}
