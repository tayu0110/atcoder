use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, mut a: [[usize; 2]; n]}

    let mut dp = vec![vec![0; 2]; n];

    dp[0][0] = 1;
    dp[0][1] = 1;

    for i in 1..n {
        for j in 0..2 {
            for k in 0..2 {
                if a[i][k] != a[i - 1][j] {
                    dp[i][k] += dp[i - 1][j];
                    dp[i][k] %= MOD;
                }
            }
        }
    }

    println!("{}", (dp[n - 1][0] + dp[n - 1][1]) % MOD)
}
