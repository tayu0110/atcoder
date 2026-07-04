use proconio::*;

fn main() {
    input! {h: usize, w: usize, k: usize, a: [[usize; w]; h]}

    let mut res = usize::MAX;
    for i in 0..h {
        for j in 0..w {
            let min = a[i][j];
            let mut a = a.clone();
            for k in 0..h {
                for l in 0..w {
                    if a[k][l] < min {
                        a[k][l] = 0;
                    }
                }
            }

            let mut dp = vec![vec![vec![usize::MAX; w]; h]; k + 1];
            if a[0][0] <= min {
                dp[0][0][0] = 0;
            }
            if a[0][0] >= min {
                dp[1][0][0] = a[0][0];
            }

            for l in 0..h {
                for m in 0..w {
                    for n in 0..=k {
                        if dp[n][l][m] == usize::MAX {
                            continue;
                        }
                        if l + 1 < h {
                            if a[l + 1][m] <= min {
                                dp[n][l + 1][m] = dp[n][l + 1][m].min(dp[n][l][m]);
                            }
                            if n + 1 <= k && a[l + 1][m] >= min {
                                dp[n + 1][l + 1][m] =
                                    dp[n + 1][l + 1][m].min(dp[n][l][m] + a[l + 1][m]);
                            }
                        }
                        if m + 1 < w {
                            if a[l][m + 1] <= min {
                                dp[n][l][m + 1] = dp[n][l][m + 1].min(dp[n][l][m]);
                            }
                            if n + 1 <= k && a[l][m + 1] >= min {
                                dp[n + 1][l][m + 1] =
                                    dp[n + 1][l][m + 1].min(dp[n][l][m] + a[l][m + 1]);
                            }
                        }
                    }
                }
            }
            res = res.min(dp[k][h - 1][w - 1]);
        }
    }

    println!("{res}")
}
