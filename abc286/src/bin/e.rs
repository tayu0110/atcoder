#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

const MAX: i64 = std::i64::MAX >> 10;

fn main() {
    input! {n: usize, a: [i64; n], s: [Chars; n], q: usize, p: [(usize, usize); q]}

    let mut dp = vec![vec![(MAX, MAX); n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                dp[i][j] = (0, 0);
            } else if s[i][j] == 'Y' {
                dp[i][j] = (1, -a[j]);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dp[i][j].0 > dp[i][k].0 + dp[k][j].0 {
                    dp[i][j] = (dp[i][k].0 + dp[k][j].0, dp[i][k].1 + dp[k][j].1);
                } else if dp[i][j].0 == dp[i][k].0 + dp[k][j].0
                    && dp[i][j].1 > dp[i][k].1 + dp[k][j].1
                {
                    dp[i][j] = (dp[i][k].0 + dp[k][j].0, dp[i][k].1 + dp[k][j].1);
                }
            }
        }
    }

    for (u, v) in p {
        let (u, v) = (u - 1, v - 1);

        let (d, c) = dp[u][v];
        if d == MAX {
            println!("Impossible")
        } else {
            println!("{} {}", d, -c + a[u]);
        }
    }
}
