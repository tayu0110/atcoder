use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut dp = vec![vec![0u16; n + 1]; n + 1];
    for (l, &c) in s.iter().enumerate() {
        for (r, &d) in s.iter().rev().enumerate() {
            if c == d {
                dp[l + 1][r + 1] = dp[l][r] + 1;
            } else {
                dp[l + 1][r + 1] = dp[l + 1][r].max(dp[l][r + 1]);
            }
        }
    }

    println!("{}", dp[n][n]);
}
