use std::cmp::Reverse;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [[u16; m]; n]}
    let mut t = vec![0; m];
    for a in a {
        for (i, a) in a.into_iter().enumerate() {
            t[i] += a;
        }
    }

    let mut t = t
        .into_iter()
        .enumerate()
        .map(|(i, t)| (t, i))
        .collect::<Vec<_>>();
    t.sort_unstable_by_key(|(t, i)| (Reverse(*t), *i));
    println!("{}", t.into_iter().map(|t| t.1 + 1).join(" "));
}
