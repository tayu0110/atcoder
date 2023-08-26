use proconio::*;

fn main() {
    input! {n: usize, mut f: [[i32; n]; n]}

    let mut dp = vec![vec![std::i32::MAX; n]; n];
    dp[0][0] = 0;
    for i in 1..n {
        let mut sum = 0;
        for j in (0..=i).rev() {
            sum += f[i][j];
            for k in 0..j {
                dp[i][j] = dp[i][j].max(dp[i - 1][k] + sum);
            }
        }
    }

    println!("{}", dp[n - 1].iter().max().unwrap())
}
