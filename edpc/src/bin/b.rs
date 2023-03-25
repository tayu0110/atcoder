use proconio::input;

fn main() {
    input! {n: usize, k: usize, h: [i32; n]}

    let mut dp = vec![std::i32::MAX; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        for j in (i + 1..=i + k).take_while(|&j| j < n) {
            dp[j] = std::cmp::min(dp[j], dp[i] + (h[i] - h[j]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
