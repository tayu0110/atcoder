use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, h: [usize; n]}

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;
    let mut prev = vec![0; n];

    for i in 0..n - 1 {
        if dp[i + 1] > dp[i] + h[i].abs_diff(h[i + 1]) {
            dp[i + 1] = dp[i] + h[i].abs_diff(h[i + 1]);
            prev[i + 1] = i;
        }

        if i + 2 < n && dp[i + 2] > dp[i] + h[i].abs_diff(h[i + 2]) {
            dp[i + 2] = dp[i] + h[i].abs_diff(h[i + 2]);
            prev[i + 2] = i;
        }
    }

    let mut res = vec![n];
    let mut now = n - 1;
    while now > 0 {
        res.push(prev[now] + 1);
        now = prev[now];
    }

    println!("{}", res.len());
    println!("{}", res.iter().rev().join(" "))
}
