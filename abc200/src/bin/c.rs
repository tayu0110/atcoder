#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut map = vec![0i64; 200];
    for a in a {
        map[a % 200] += 1;
    }

    println!("{}", map.into_iter().map(|v| v * (v - 1) / 2).sum::<i64>());
}
