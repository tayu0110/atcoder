use proconio::input;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], b: [usize; m]}

    let mut dp = vec![vec![std::usize::MAX / 4; m+1]; n+1];
    for i in 0..n {
        dp[i][0] = i;
    }
    for j in 0..m {
        dp[0][j] = j;
    }

    for i in 1..=n {
        for j in 1..=m {
            dp[i][j] = std::cmp::min(dp[i][j], std::cmp::min(dp[i-1][j] + 1, dp[i][j-1] + 1));
            if a[i-1] == b[j-1] {
                dp[i][j] = std::cmp::min(dp[i][j], dp[i-1][j-1]);
            } else {
                dp[i][j] = std::cmp::min(dp[i][j], dp[i-1][j-1] + 1);
            }
        }
    }

    println!("{}", dp[n][m]);
}