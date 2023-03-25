use proconio::*;

fn main() {
    input! {n: usize, a: [[usize; n]; 2]}

    let mut dp = vec![vec![0; n]; 2];
    dp[0][0] = a[0][0];
    for i in 0..2 {
        for j in 0..n {
            if i + 1 < 2 {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + a[i + 1][j]);
            }
            if j + 1 < n {
                dp[i][j + 1] = dp[i][j + 1].max(dp[i][j] + a[i][j + 1]);
            }
        }
    }

    println!("{}", dp[1][n - 1])
}
