use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut dp = vec![vec![0; n + 1]; n + 1];
    for l in (1..=n).rev() {
        dp[l][l] = 1;
        for r in l + 1..=n {
            dp[l][r] = dp[l + 1][r];
            for k in l + 1..r {
                if a[l] < a[k] {
                    dp[l][r] += dp[l + 1][k] * dp[k][r] % MOD;
                    dp[l][r] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[1][n])
}
