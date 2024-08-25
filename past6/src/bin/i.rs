use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[i64; w]; h]}

    let mut dp = vec![[[-1; 200]; 100]; 100];
    dp[0][0][0] = 0;
    dp[0][0][1] = a[0][0];
    for i in 0..h {
        for j in 0..w {
            for k in 0..h + w {
                if dp[i][j][k] < 0 {
                    continue;
                }

                if i + 1 < h {
                    dp[i + 1][j][k] = dp[i + 1][j][k].max(dp[i][j][k]);
                    dp[i + 1][j][k + 1] = dp[i + 1][j][k + 1].max(dp[i][j][k] + a[i + 1][j]);
                }
                if j + 1 < w {
                    dp[i][j + 1][k] = dp[i][j + 1][k].max(dp[i][j][k]);
                    dp[i][j + 1][k + 1] = dp[i][j + 1][k + 1].max(dp[i][j][k] + a[i][j + 1]);
                }
            }
        }
    }

    for i in 1..h + w {
        println!("{}", dp[h - 1][w - 1][i]);
    }
}
