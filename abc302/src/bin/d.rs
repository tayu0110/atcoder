use std::collections::BTreeSet;

use proconio::*;

fn solve(d: i64, a: &[i64], b: &[i64]) -> i64 {
    let mut res = -1;
    let mut set = BTreeSet::new();
    for &b in b {
        set.insert(b);
    }

    for &a in a {
        if let Some(&t) = set.range(..=a + d).next_back() {
            if (t - a).abs() <= d {
                res = res.max(t + a);
            }
        }
    }

    res
}

fn main() {
    input! {n: usize, m: usize, d: i64, mut a: [i64; n], mut b: [i64; m]}

    a.sort();
    b.sort();
    println!("{}", solve(d, &a, &b).max(solve(d, &b, &a)))
}
