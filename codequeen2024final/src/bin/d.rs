use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, e: [(usize, usize, usize); m], mut event: [(usize, usize); k]}

    event.sort_unstable_by_key(|e| e.1);
    event.insert(0, (1, 0));

    let mut t = vec![vec![]; n];
    for (a, b, d) in e {
        t[a - 1].push((b - 1, d));
        t[b - 1].push((a - 1, d));
    }

    let mut dist = vec![];
    for start in 0..n {
        let mut d = vec![usize::MAX; n];
        let mut nt = BinaryHeap::new();
        nt.push(Reverse((0, start)));
        while let Some(Reverse((nd, now))) = nt.pop() {
            if d[now] < usize::MAX {
                continue;
            }
            d[now] = nd;
            for &(to, c) in &t[now] {
                if d[to] == usize::MAX {
                    nt.push(Reverse((nd + c, to)));
                }
            }
        }

        dist.push(d);
    }

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 0..k {
        if dp[i] == 0 {
            continue;
        }
        let (c, t) = event[i];

        for j in i + 1..=k {
            let (nc, nt) = event[j];
            if t.saturating_add(dist[c - 1][nc - 1]) <= nt {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap() - 1);
}
