#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, mut s: [Chars; n]}

    let mut map = std::collections::HashMap::new();
    for v in s.iter_mut() {
        v.sort();
        *map.entry(v).or_insert(0usize) += 1;
    }

    let mut res = 0;
    for (_, v) in map {
        res += v * (v - 1) / 2;
    }
    println!("{}", res);
}
