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
    let mut res = vec![];
    for (k, v) in map {
        if v > max {
            max = v;
            res = vec![k];
        } else if v == max {
            res.push(k);
        }
    }

    res.sort();

    for res in res {
        println!("{}", res);
    }
}
