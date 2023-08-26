use proconio::*;

const MAX: usize = std::u32::MAX as usize;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut dp = vec![vec![MAX; n]; n];
    for (a, b, c) in p {
        dp[a - 1][b - 1] = c;
    }
    for i in 0..n {
        dp[i][i] = 0;
    }

    let mut res = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
                if dp[i][j] < MAX {
                    res += dp[i][j];
                }
            }
        }
    }

    println!("{}", res)
}
