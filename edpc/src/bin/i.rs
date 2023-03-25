use proconio::input;

fn main() {
    input! {n: usize, p: [f64; n]}

    let mut dp = vec![0.0; n + 1];
    dp[0] = 1.0;

    for p in p {
        let mut new = vec![0.0; n + 1];
        for i in 0..n {
            new[i + 1] += dp[i] * p;
            new[i] += dp[i] * (1.0 - p);
        }

        dp = new;
    }

    println!("{:.10}", dp.into_iter().skip(n / 2 + 1).sum::<f64>())
}
