#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, k: usize, s: [String; n]}

    let mut map = std::collections::HashMap::new();
    for s in s {
        *map.entry(s).or_insert(0) += 1;
    }

    let mut t = vec![vec![]; n + 1];

    for (s, v) in map {
        t[v].push(s);
    }

    let mut now = 0;
    for v in t.into_iter().rev() {
        if v.is_empty() {
            continue;
        }

        if now + v.len() >= k {
            if v.len() > 1 {
                println!("AMBIGUOUS");
            } else {
                println!("{}", v[0]);
            }
            return;
        }

        now += v.len();
    }
}
