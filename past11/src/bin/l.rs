use proconio::*;

fn main() {
    input! {s: marker::Bytes, t: marker::Bytes, k: usize}

    let mut dp = vec![vec![vec![0; k + 1]; t.len() + 1]; s.len() + 1];
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            for nk in (0..=k).rev() {
                dp[i][j][nk] = dp[i - 1][j][nk].max(dp[i][j - 1][nk]);
                if s[i - 1] == t[j - 1] {
                    dp[i][j][nk] = dp[i][j][nk].max(dp[i - 1][j - 1][nk] + 1);
                } else if nk < k {
                    dp[i][j][nk + 1] = dp[i][j][nk + 1].max(dp[i - 1][j - 1][nk] + 1);
                }
            }
        }
    }

    println!("{}", dp[s.len()][t.len()].iter().max().unwrap())
}
