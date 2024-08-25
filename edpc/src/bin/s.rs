use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {k: Chars, d: usize}

    let n = k.len();
    let mut dp = vec![vec![vec![0; 2]; d]; n + 1];
    dp[0][0][1] = 1;

    for i in 0..n {
        for j in 0..d {
            for l in 0..10 {
                let nj = (j + l) % d;
                dp[i + 1][nj][0] += dp[i][j][0];
                dp[i + 1][nj][0] %= MOD;
                if l == k[i] as usize - b'0' as usize {
                    dp[i + 1][nj][1] += dp[i][j][1];
                    dp[i + 1][nj][1] %= MOD;
                } else if l < k[i] as usize - b'0' as usize {
                    dp[i + 1][nj][0] += dp[i][j][1];
                    dp[i + 1][nj][0] %= MOD;
                }
            }
        }
    }

    println!("{}", (dp[n][0][0] + dp[n][0][1] + MOD - 1) % MOD)
}
