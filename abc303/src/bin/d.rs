use proconio::*;

fn main() {
    input! {x: usize, y: usize, z: usize, s: marker::Chars}
    let n = s.len();

    let mut dp = vec![vec![std::usize::MAX; n + 1]; 2];
    dp[0][0] = 0;
    for (i, c) in s.into_iter().enumerate() {
        dp[1][i] = dp[1][i].min(dp[0][i] + z);
        dp[0][i] = dp[0][i].min(dp[1][i] + z);

        if c == 'a' {
            dp[0][i + 1] = dp[0][i] + x;
            dp[1][i + 1] = dp[1][i] + y;
        } else {
            dp[0][i + 1] = dp[0][i] + y;
            dp[1][i + 1] = dp[1][i] + x;
        }
    }

    println!("{}", std::cmp::min(dp[0][n], dp[1][n]))
}
