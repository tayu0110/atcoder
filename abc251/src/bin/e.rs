use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut dp = vec![vec![vec![std::usize::MAX; 2]; 2]; n + 1];
    dp[0][0][0] = a[n - 1];
    dp[0][1][1] = a[0];
    for i in 1..n {
        for j in 0..2 {
            for k in 0..2 {
                if j == 0 {
                    dp[i][j][k] = dp[i][j][k].min(dp[i - 1][1][k]);
                } else {
                    let t = if k == 0 && i == n - 1 { 0 } else { a[i] };
                    dp[i][j][k] = dp[i][j][k].min(dp[i - 1][1][k].saturating_add(t));
                    dp[i][j][k] = dp[i][j][k].min(dp[i - 1][0][k].saturating_add(t));
                }
            }
        }
    }

    println!(
        "{}",
        dp[n - 1][0][1].min(dp[n - 1][1][0]).min(dp[n - 1][1][1])
    )
}
