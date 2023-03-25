#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, t: usize, p: [(usize, usize); n]}

    let mut res = std::usize::MAX;
    for (c, d) in p {
        if d <= t {
            res = std::cmp::min(res, c);
        }
    }

    if res == std::usize::MAX {
        println!("TLE")
    } else {
        println!("{}", res)
    }
}
