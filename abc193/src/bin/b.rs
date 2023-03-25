#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(usize, usize, usize); n]}

    let mut res = std::usize::MAX;
    for (a, p, x) in p {
        if a < x {
            res = std::cmp::min(res, p);
        }
    }

    if res == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
