#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    println!("{}", a.into_iter().chain(vec![0; std::cmp::max(k, n)].into_iter()).skip(k).take(n).join(" "));
}
