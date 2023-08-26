use proconio::*;

fn main() {
    input! {k: usize, r: [f64; 1 << k]}

    let mut dp = vec![vec![0.0; k + 1]; 1 << k];
    for i in 0..1 << k {
        dp[i][0] = 1.0;
    }

    let f = |i: f64, j: f64| 1.0 / (1.0 + 10.0f64.powf((j - i) / 400.0));
    for j in 1..=k {
        for i in 0..1 << k {
            let op = 1 << (j - 1);
            let base = (i ^ op) & !(op - 1);
            let mut sum = 0.0;
            for k in base..base + op {
                sum += dp[k][j - 1] * f(r[i], r[k]);
            }

            dp[i][j] = dp[i][j - 1] * sum;
        }
    }

    for i in 0..1 << k {
        println!("{}", dp[i][k]);
    }
}
