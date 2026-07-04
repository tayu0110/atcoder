use proconio::*;

fn main() {
    input! {n: usize, x: usize, q: [(usize, usize, usize); n]}

    let mut dp = vec![vec![0.0f64; x + 1]; 1 << n];
    for j in 0..=x {
        for i in 0..1 << n {
            for (k, &(s, c, p)) in q.iter().enumerate() {
                if i & (1 << k) != 0 {
                    continue;
                }

                if j < c {
                    continue;
                }

                dp[i][j] = dp[i][j].max(
                    (dp[i | 1 << k][j - c] + s as f64) * p as f64 / 100.0
                        + dp[i][j - c] * (100 - p) as f64 / 100.0,
                );
            }
        }
    }

    println!("{}", dp[0][x])
}
