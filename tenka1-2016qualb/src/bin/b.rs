use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let len = s.len();

    let mut dp = vec![vec![std::usize::MAX; len]; len + 1];
    dp[0][0] = 0;

    for (i, c) in s.into_iter().enumerate() {
        let mut new = vec![vec![std::usize::MAX; len]; len + 1];
        for j in 0..len {
            for k in 0..len {
                if dp[j][k] == std::usize::MAX {
                    continue;
                }
                if c == '(' {
                    new[j + 1][k] = new[j + 1][k].min(dp[j][k]);
                    if j > 0 {
                        new[j - 1][i] = new[j - 1][i].min(dp[j][k] + i - k + 1);
                    }
                } else {
                    if j > 0 {
                        new[j - 1][k] = new[j - 1][k].min(dp[j][k]);
                    }
                    new[j + 1][i] = new[j + 1][i].min(dp[j][k] + i - k + 1);
                }
            }
        }
        dp = new;
    }

    let mut res = std::usize::MAX;
    for i in 0..len {
        res = res.min(dp[0][i]);
    }
    println!("{}", res)
}
