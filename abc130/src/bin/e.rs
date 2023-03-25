use proconio::input;

fn main() {
    input! {n: usize, m: usize, s: [usize; n], t: [usize; m]};
    const MOD: i64 = 1000000007;

    let mut dp = vec![vec![0i64; m+1]; n+1];
    dp[0][0] = 1;
    for i in 0..=n {
        for j in 0..=m {
            if i > 0 && j > 0 && s[i-1] == t[j-1] {
                dp[i][j] += dp[i-1][j-1];
            }

            if i > 0 {
                dp[i][j] += dp[i-1][j];
            }
            if j > 0 {
                dp[i][j] += dp[i][j-1];
            }
            if i > 0 && j > 0 {
                dp[i][j] -= dp[i-1][j-1];
            }

            dp[i][j] = (dp[i][j] % MOD + MOD) % MOD;
        }
    }

    println!("{}", dp[n][m]);
}