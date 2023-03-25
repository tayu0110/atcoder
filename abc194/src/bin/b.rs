#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut res = p.iter().cloned().map(|(a, b)| a + b).min().unwrap();
    for (i, &(a, b)) in p.iter().enumerate() {
        for &(pa, pb) in p.iter().skip(i+1) {
            res = std::cmp::min(res, std::cmp::min(std::cmp::max(a, pb), std::cmp::max(b, pa)));
        }
    }

    println!("{}", res);
}
