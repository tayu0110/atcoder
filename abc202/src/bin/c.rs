#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n], c: [usize; n]}

    let mut map = std::collections::HashMap::new();
    for c in c {
        *map.entry(b[c - 1]).or_insert(0) += 1;
    }

    let res = a
        .into_iter()
        .fold(0usize, |s, v| s + *map.get(&v).unwrap_or(&0));
    println!("{}", res);
}
