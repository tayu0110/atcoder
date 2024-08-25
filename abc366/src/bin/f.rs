use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut f: [(usize, usize); n]}

    f.sort_unstable_by(|f0, f1| (f0.0 * f1.1 + f0.1).cmp(&(f1.0 * f0.1 + f1.1)));

    let mut dp = vec![vec![0; n + 1]; k + 1];
    dp[0] = vec![1; n];
    for i in 0..k {
        for (j, &(a, b)) in f.iter().enumerate() {
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(a * dp[i][j] + b);
            dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i + 1][j]);
        }
    }

    println!("{}", dp[k][n]);
}
