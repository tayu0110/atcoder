#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {s: Chars}

    let mut map = std::collections::HashMap::new();
    for v in s.windows(2) {
        *map.entry(v.into_iter().collect::<String>()).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut s = "zzzzzzzz".to_string();
    for (k, v) in map {
        if max < v {
            max = v;
            s = k;
        } else if max == v {
            s = std::cmp::min(s, k);
        }
    }

    println!("{}", s);
}
