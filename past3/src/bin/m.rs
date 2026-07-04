use std::{collections::VecDeque, usize};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], s: usize, k: usize, mut t: [usize; k]}
    t.insert(0, s);
    t.iter_mut().for_each(|t| *t -= 1);

    let mut g = vec![vec![]; n];
    for (u, v) in e {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let k = t.len();
    let mut dist = vec![];
    for &now in &t {
        let mut d = vec![u32::MAX; n];
        d[now] = 0;
        let mut nt = VecDeque::new();
        nt.push_back(now);
        while let Some(now) = nt.pop_front() {
            for &to in &g[now] {
                if d[to] == u32::MAX {
                    d[to] = d[now] + 1;
                    nt.push_back(to);
                }
            }
        }

        let mut nd = vec![0; k];
        for (i, &nt) in t.iter().enumerate() {
            nd[i] = d[nt];
        }
        dist.push(nd);
    }

    let mut dp = vec![vec![u32::MAX; 1 << k]; k];
    dp[0][1] = 0;
    for i in 1..1 << k {
        for j in 0..k {
            if dp[j][i] == u32::MAX {
                continue;
            }

            if i & (1 << j) == 0 {
                continue;
            }

            for k in 0..k {
                if i & (1 << k) != 0 {
                    continue;
                }

                dp[k][i | (1 << k)] = dp[k][i | (1 << k)].min(dp[j][i] + dist[j][k]);
            }
        }
    }

    println!("{}", dp.iter().map(|d| d[(1 << k) - 1]).min().unwrap())
}
