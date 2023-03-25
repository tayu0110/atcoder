#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn solve(v: Vec<usize>) -> Vec<(usize, usize)> {
    let mut map = std::collections::HashMap::new();
    for v in v {
        *map.entry(v).or_insert(0) += 1;
    }

    map.into_iter().map(|(k, v)| (v, k)).collect()
}

fn main() {
    input! {n: usize, v: [usize; n]}

    let v1 = v
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, &v)| v)
        .collect::<Vec<_>>();
    let v2 = v
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 1)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();

    let k: Vec<(usize, usize)> = {
        let mut t = solve(v1);
        t.sort_by_key(|&v| std::cmp::Reverse(v));

        t.into_iter().take(2).collect()
    };

    let l: Vec<(usize, usize)> = {
        let mut t = solve(v2);
        t.sort_by_key(|&v| std::cmp::Reverse(v));

        t.into_iter().take(2).collect()
    };

    let mut res = std::usize::MAX;

    for &(v1, k1) in &k {
        for &(v2, k2) in &l {
            if k1 == k2 {
                continue;
            }

            res = std::cmp::min(res, n - v1 - v2);
        }
    }

    if res == std::usize::MAX {
        println!("{}", n / 2)
    } else {
        println!("{}", res);
    }
}
