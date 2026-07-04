use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m], k: usize, t: usize, d: [usize; k], q: usize}

    let mut dp = vec![vec![usize::MAX; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for (a, b, c) in e {
        dp[a - 1][b - 1] = dp[a - 1][b - 1].min(c);
        dp[b - 1][a - 1] = dp[b - 1][a - 1].min(c);
    }
    for i in 0..k {
        let u = d[i] - 1;
        dp[u][n] = 0;
        dp[n][u] = t;
    }

    for k in 0..n + 1 {
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
            }
        }
    }

    let update = |tar: [usize; 2], dp: &mut Vec<Vec<usize>>| {
        for k in tar {
            for i in 0..n + 1 {
                for j in 0..n + 1 {
                    dp[i][j] = dp[i][j].min(dp[i][k].saturating_add(dp[k][j]));
                }
            }
        }
    };

    let mut ans = vec![];
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize, y: usize, t: usize}
            dp[x - 1][y - 1] = dp[x - 1][y - 1].min(t);
            dp[y - 1][x - 1] = dp[y - 1][x - 1].min(t);
            update([x - 1, y - 1], &mut dp);
        } else if ty == 2 {
            input! {x: usize}
            dp[x - 1][n] = 0;
            dp[n][x - 1] = t;
            update([x - 1, n], &mut dp);
        } else {
            let mut res = 0;
            for i in 0..n {
                for j in 0..n {
                    if dp[i][j] < usize::MAX {
                        res += dp[i][j];
                    }
                }
            }
            ans.push(res);
        }
    }

    println!("{}", ans.iter().join("\n"))
}
