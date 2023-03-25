#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut dp = vec![vec![vec![std::usize::MAX; 2]; 2]; n];

    dp[0][0][0] = a[0];
    dp[0][1][1] = 0;

    for i in 0..n-1 {
        for j in 0..2 {
            for k in 0..2 {
                if dp[i][j][k] == std::usize::MAX {
                    continue;
                }
                if j == 0 {
                    dp[i+1][0][k] = std::cmp::min(dp[i+1][0][k], dp[i][j][k] + a[i+1] + b[i]);
                    dp[i+1][1][k] = std::cmp::min(dp[i+1][1][k], dp[i][j][k]);
                } else {
                    dp[i+1][0][k] = std::cmp::min(dp[i+1][0][k], dp[i][j][k] + a[i+1]);
                    dp[i+1][1][k] = std::cmp::min(dp[i+1][1][k], dp[i][j][k] + b[i]);
                }
            }
        }
    }

    println!("{}", std::cmp::min(dp[n-1][0][0] + b[n-1], std::cmp::min(dp[n-1][0][1], std::cmp::min(dp[n-1][1][0], dp[n-1][1][1] + b[n-1]))));
}
