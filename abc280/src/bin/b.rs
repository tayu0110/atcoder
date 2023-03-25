#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [i64; n]}

    let mut res = vec![];
    let mut sum = 0;
    for k in s {
        res.push(k - sum);
        sum = k;
    }

    println!("{}", res.iter().join(" "));
}
