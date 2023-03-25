#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut map = std::collections::HashMap::new();
    for a in a {
        *map.entry(a).or_insert(0i64) += 1;
    }

    let mut res = 0;
    for (&k, &v) in &map {
        for (&pk, &pv) in &map {
            res += (pk - k) * (pk - k) * v * pv;
        }
    }

    println!("{}", res / 2);
}
