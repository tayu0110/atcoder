use proconio::*;

fn main() {
    input! {n: usize, c: i64, a: [i64; n]}

    let mut dp = vec![vec![i64::MAX; n + 1]; n + 1];
    for i in 0..=n {
        dp[i][i] = 0;
    }
    for i in 0..n {
        dp[i][i + 1] = c;
    }

    for len in 1..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                continue;
            }
            if l + 1 <= r - 1 {
                dp[l][r] = dp[l][r].min(dp[l + 1][r - 1] + c + (a[l] - a[r - 1]).abs());
            }
            for mid in l..=r {
                dp[l][r] = dp[l][r].min(dp[l][mid] + dp[mid][r] - c);
            }
        }
    }

    println!("{}", dp[0][n])
}
