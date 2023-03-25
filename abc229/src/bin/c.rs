#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, mut w: usize, mut p: [(usize, usize); n]}
    p.sort_by_key(|(a, _)| std::cmp::Reverse(*a));

    let mut res = 0;
    for (a, b) in p {
        if b <= w {
            res += a * b;
            w -= b;
        } else {
            res += a * w;
            w = 0;
        }
    }

    println!("{}", res);
}
