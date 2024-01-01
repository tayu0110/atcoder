use std::{cmp::Reverse, collections::HashSet};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, l: usize, a: [usize; n], b: [usize; m], p: [(usize, usize); l]}

    let mut b = b.into_iter().enumerate().collect::<Vec<_>>();
    b.sort_unstable_by_key(|v| Reverse(v.1));

    let mut bad = vec![HashSet::new(); n];
    for (c, d) in p {
        bad[c - 1].insert(d - 1);
    }

    let mut res = 0;
    for (i, a) in a.into_iter().enumerate() {
        for &(j, b) in &b {
            if bad[i].contains(&j) {
                continue;
            }

            res = res.max(a + b);
            break;
        }
    }

    println!("{res}")
}
