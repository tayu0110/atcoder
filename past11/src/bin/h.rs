use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, p: [(usize, i64); n]}

    let mut dp = vec![vec![-1; b + 1]; a + 1];
    dp[0][0] = 0;
    for (w, v) in p {
        for i in (0..=a).rev() {
            for j in (0..=b).rev() {
                if dp[i][j] < 0 {
                    continue;
                }
                if i + w <= a {
                    dp[i + w][j] = dp[i + w][j].max(dp[i][j] + v);
                }
                if j + w <= b {
                    dp[i][j + w] = dp[i][j + w].max(dp[i][j] + v);
                }
            }
        }
    }

    println!("{}", dp.iter().flatten().max().unwrap())
}
