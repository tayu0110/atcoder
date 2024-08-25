use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: usize, p: [(usize, usize, usize, usize); m]}

    let mut memo = vec![BinaryHeap::<Reverse<(usize, usize)>>::new(); n];
    memo[p[0].1 - 1].push(Reverse((p[0].3, p[0].3 + x)));
    let mut max = vec![0usize; n];

    let mut p = p.into_iter().enumerate().collect::<Vec<_>>();
    p.sort_unstable_by_key(|p| (p.1 .2, p.1 .3));

    let mut res = vec![0; m];
    for (i, (a, b, s, t)) in p {
        if i == 0 {
            continue;
        }

        while let Some(&Reverse((_, otx))) = memo[a - 1].peek().filter(|&&Reverse(m)| m.0 <= s) {
            memo[a - 1].pop();
            max[a - 1] = max[a - 1].max(otx);
        }

        res[i] = max[a - 1].saturating_sub(s);

        memo[b - 1].push(Reverse((t, t + res[i])));
    }

    println!("{}", res[1..].iter().join(" "))
}
