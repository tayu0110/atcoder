use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, n: usize}

    let mut good = HashSet::new();
    let mut keep = vec![];
    for _ in 0..n {
        input! {i: usize, j: usize, k: usize, r: usize}

        if r == 1 {
            good.insert(i);
            good.insert(j);
            good.insert(k);
        } else {
            keep.push([i, j, k]);
        }
    }

    let mut bad = HashSet::new();
    for v in keep {
        let (_, b): (Vec<_>, Vec<_>) = v.into_iter().partition(|v| good.contains(v));
        if b.len() == 1 {
            bad.insert(b[0]);
        }
    }

    println!(
        "{}",
        (1..=a + b + c)
            .map(|i| {
                if good.contains(&i) {
                    1
                } else if bad.contains(&i) {
                    0
                } else {
                    2
                }
            })
            .join("\n")
    );
}
