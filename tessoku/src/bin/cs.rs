use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes}

    let m = t.len();
    let mut dp = [[u16::MAX; 2001]; 2];
    dp[0][0] = 0;

    for s in s {
        for (j, &t) in t.iter().enumerate() {
            dp[1][j] = dp[1][j].min(dp[0][j] + 1);
            dp[0][j + 1] = dp[0][j + 1].min(dp[0][j] + 1);
            dp[1][j + 1] = dp[1][j + 1].min(dp[0][j] + 1 - (s == t) as u16);
        }

        dp[1][m] = dp[1][m].min(dp[0][m] + 1);

        dp.swap(0, 1);
        dp[1].fill(u16::MAX);
    }

    for j in 0..m {
        dp[0][j + 1] = dp[0][j + 1].min(dp[0][j] + 1);
    }

    println!("{}", dp[0][m])
}
