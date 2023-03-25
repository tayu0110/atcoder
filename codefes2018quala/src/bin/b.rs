#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, a: usize, b: usize, p: [(usize, usize); m]}

    let mut v = vec![std::cmp::max(a, b); n];
    for (l, r) in p {
        for i in l - 1..r {
            v[i] = a;
        }
    }

    println!("{}", v.iter().sum::<usize>())
}
