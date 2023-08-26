use proconio::*;

const MOD: usize = 1000_000_007;

fn main() {
    input! {l: marker::Chars}
    let len = l.len();

    let mut dp = vec![vec![0; 2]; len];
    dp[0][0] = 1;
    dp[0][1] = 2;
    for i in 1..len {
        // 0
        dp[i][0] += dp[i - 1][0] * 3;
        dp[i][0] %= MOD;
        // 1
        if l[i] == '1' {
            dp[i][0] += dp[i - 1][1];
            dp[i][0] %= MOD;
            dp[i][1] += dp[i - 1][1] * 2;
            dp[i][1] %= MOD;
        } else {
            dp[i][1] += dp[i - 1][1];
            dp[i][1] %= MOD;
        }
    }

    println!("{}", (dp[len - 1][0] + dp[len - 1][1]) % MOD)
}
