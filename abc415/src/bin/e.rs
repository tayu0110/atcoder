use proconio::*;

fn main() {
    input! {h: usize, w: usize, a: [[i64; w]; h], p: [i64; h + w - 1]}

    let (mut l, mut r) = (-1, 1_000_000_000_000_000_000i64);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut dp = vec![vec![i64::MIN; w]; h];
        dp[0][0] = m;
        for i in 0..h {
            for j in 0..w {
                dp[i][j] += a[i][j];
                dp[i][j] = dp[i][j].saturating_sub(p[i + j]);
                if dp[i][j] < 0 {
                    continue;
                }

                if i + 1 < h {
                    dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
                }
                if j + 1 < w {
                    dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
                }
            }
        }

        if dp[h - 1][w - 1] >= 0 {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r}")
}
