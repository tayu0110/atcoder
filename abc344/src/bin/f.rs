use proconio::*;

fn main() {
    input! {n: usize, p: [[usize; n]; n], r: [[usize; n - 1]; n], d: [[usize; n]; n - 1]}

    let mut dp = vec![vec![vec![vec![(usize::MAX, usize::MAX); n]; n]; n]; n];
    dp[0][0][0][0] = (0, usize::MAX);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                for l in 0..n {
                    let cos = dp[i][j][k][l].0;
                    let mon = usize::MAX - dp[i][j][k][l].1;

                    if cos == usize::MAX {
                        continue;
                    }
                    if i + 1 < n {
                        let (nk, nl) = if p[k][l] > p[i + 1][j] {
                            (k, l)
                        } else {
                            (i + 1, j)
                        };

                        let mut cos = cos + 1;
                        let mut mon = mon;
                        let nm = (d[i][j].saturating_sub(mon) + p[k][l] - 1) / p[k][l];
                        cos += nm;
                        mon += p[k][l] * nm;
                        assert!(mon >= d[i][j]);
                        mon -= d[i][j];

                        dp[i + 1][j][nk][nl] = dp[i + 1][j][nk][nl].min((cos, usize::MAX - mon));
                    }

                    if j + 1 < n {
                        let (nk, nl) = if p[k][l] > p[i][j + 1] {
                            (k, l)
                        } else {
                            (i, j + 1)
                        };

                        let mut cos = cos + 1;
                        let mut mon = mon;
                        let nm = (r[i][j].saturating_sub(mon) + p[k][l] - 1) / p[k][l];
                        cos += nm;
                        mon += p[k][l] * nm;
                        assert!(mon >= r[i][j]);
                        mon -= r[i][j];
                        dp[i][j + 1][nk][nl] = dp[i][j + 1][nk][nl].min((cos, usize::MAX - mon));
                    }
                }
            }
        }
    }

    let mut res = usize::MAX;
    for i in 0..n {
        for j in 0..n {
            res = res.min(dp[n - 1][n - 1][i][j].0);
        }
    }

    println!("{}", res);
}
