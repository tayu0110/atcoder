#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, s: [String; n]}

    let mut map = std::collections::HashMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut res = "".to_string();
    for (k, v) in map {
        if max < v {
            max = v;
            res = k;
        }
    }

    println!("{}", res);
}
