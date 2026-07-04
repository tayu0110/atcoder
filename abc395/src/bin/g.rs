use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, k: usize, c: [[usize; n]; n], q: usize, query: [(usize, usize); q]}

    let mut dist = c.clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut res = vec![vec![usize::MAX; n]; n];
    for s in k..n {
        let mut dp = vec![vec![usize::MAX; n]; 1 << (k + 1)];
        for i in 0..k {
            dp[1 << i][i] = 0;
        }
        dp[1 << k][s] = 0;

        let mut nt = BinaryHeap::new();
        for i in 1..1 << (k + 1) {
            for j in 0..n {
                let mut now = i;
                while now > 0 {
                    dp[i][j] = dp[i][j].min(dp[now][j].saturating_add(dp[i ^ now][j]));
                    now = (now - 1) & i;
                }
            }

            for j in 0..n {
                nt.push(Reverse((dp[i][j], j)));
            }

            while let Some(Reverse((nd, now))) = nt.pop() {
                if dp[i][now] < nd {
                    continue;
                }
                dp[i][now] = nd;
                for to in 0..n {
                    if dp[i][to] > dp[i][now].saturating_add(dist[now][to]) {
                        dp[i][to] = dp[i][now] + dist[now][to];
                        nt.push(Reverse((dp[i][to], to)));
                    }
                }
            }
        }

        for t in 0..n {
            res[s][t] = dp[(1 << (k + 1)) - 1][t];
        }
    }

    println!(
        "{}",
        query.into_iter().map(|(s, t)| res[s - 1][t - 1]).join("\n")
    );
}
