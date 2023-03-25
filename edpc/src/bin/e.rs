use proconio::input;

fn main() {
    input! {n: usize, w: i32, p: [(i32, usize); n]}
    const V_MAX: usize = 100000;

    let mut dp = vec![std::i32::MAX; V_MAX + 1];
    dp[0] = 0;

    for (u, v) in p {
        for from in (0..dp.len() - v).rev() {
            if dp[from] != std::i32::MAX && dp[from] + u <= w {
                dp[from + v] = std::cmp::min(dp[from + v], dp[from] + u);
            }
        }
    }

    println!(
        "{}",
        dp.into_iter()
            .enumerate()
            .filter(|&(_, v)| v <= w)
            .map(|(i, _)| i)
            .last()
            .unwrap()
    )
}
