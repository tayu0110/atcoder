use proconio::input;

fn main() {
    input! {n: usize, h: [i32; n]}

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + (h[i] - h[i + 1]).abs());
        if i + 2 < n {
            dp[i + 2] = std::cmp::min(dp[i + 2], dp[i] + (h[i] - h[i + 2]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
