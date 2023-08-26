use proconio::*;

const MIN: i64 = -(std::i64::MAX >> 10);

fn main() {
    input! {n: usize, p: [(usize, i64); n]}

    // 0: health, 1: bad
    let mut dp = vec![vec![MIN; n + 1]; 2];
    dp[0][0] = 0;
    for (i, (x, y)) in p.into_iter().enumerate() {
        for j in 0usize..2 {
            dp[j][i + 1] = dp[j][i + 1].max(dp[j][i]);
            if x == 0 {
                let next = j.saturating_sub(1);
                dp[next][i + 1] = dp[next][i + 1].max(dp[j][i] + y);
            } else {
                if j == 1 {
                    continue;
                }
                dp[j + 1][i + 1] = dp[j + 1][i + 1].max(dp[j][i] + y);
            }
        }
    }

    println!("{}", dp[0][n].max(dp[1][n]))
}
