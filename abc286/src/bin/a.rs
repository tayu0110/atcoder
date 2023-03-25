#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: usize, q: usize, r: usize, s: usize, a: [usize; n]}
    let (p, q, r, s) = (p - 1, q - 1, r - 1, s - 1);

    let res = (0..p)
        .chain((r..s + 1).chain(q + 1..r).chain(p..q + 1).chain(s + 1..n))
        .map(|v| a[v])
        .collect::<Vec<_>>();
    println!("{}", res.iter().join(" "))
}
