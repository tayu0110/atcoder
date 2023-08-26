use proconio::*;
use std::collections::HashSet;

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let a = a.into_iter().collect::<HashSet<_>>();

    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 0..n {
        if !a.contains(&(i + 1)) {
            dp[i + 1] += dp[i];
            dp[i + 1] %= MOD;
        }
        if i + 2 <= n && !a.contains(&(i + 2)) {
            dp[i + 2] += dp[i];
            dp[i + 2] %= MOD;
        }
    }

    println!("{}", dp[n])
}
