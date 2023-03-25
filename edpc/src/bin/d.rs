use proconio::input;

fn main() {
    input! {n: usize, w: usize, p: [(usize, i64); n]}

    let mut dp = vec![std::i64::MIN; w + 1];
    dp[0] = 0;

    for (w, v) in p {
        for from in (0..dp.len() - w).rev() {
            if dp[from] >= 0 {
                dp[from + w] = std::cmp::max(dp[from + w], dp[from] + v);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap());
}
