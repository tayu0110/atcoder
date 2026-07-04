use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut t = vec![vec![]; n + 1];
    for (i, h) in h.into_iter().enumerate() {
        t[h].push(i);
    }

    let mut res = vec![0i32; n];
    let mut set = BTreeSet::new();
    while let Some(mut v) = t.pop() {
        while let Some(v) = v.pop() {
            let &prev = set.range(..v).next_back().unwrap_or(&0);
            res[prev] += 1;
            res[v] -= 1;
            set.insert(v);
        }
    }

    for i in 0..n - 1 {
        res[i + 1] += res[i];
    }

    println!("{}", res.iter().join(" "))
}
