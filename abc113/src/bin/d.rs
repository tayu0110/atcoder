use proconio::*;

const MOD: usize = 1000_000_007;

fn main() {
    input! {h: usize, w: usize, k: usize}

    let mut dp = vec![vec![vec![0; w]; 1 << w]; h + 1];
    dp[0][0][0] = 1;
    for i in 0..h {
        for j in (0..1 << (w - 1)).map(|j| j << 1) {
            'K: for k in (0..1 << (w - 1)).map(|k| k << 1) {
                for l in 0..w {
                    if (k >> l) & 0b11 == 0b11 {
                        continue 'K;
                    }
                }
                for l in 0..w {
                    let (s, t) = ((k >> l) & 1, (k >> (l + 1)) & 1);
                    if s == 1 && t == 1 {
                        continue;
                    } else if s == 1 && t == 0 {
                        dp[i + 1][k][l - 1] += dp[i][j][l];
                        dp[i + 1][k][l - 1] %= MOD;
                    } else if s == 0 && t == 1 {
                        dp[i + 1][k][l + 1] += dp[i][j][l];
                        dp[i + 1][k][l + 1] %= MOD;
                    } else {
                        dp[i + 1][k][l] += dp[i][j][l];
                        dp[i + 1][k][l] %= MOD;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..1 << w {
        res += dp[h][i][k - 1];
        res %= MOD;
    }

    println!("{}", res)
}
