use proconio::*;

fn main() {
    input! {n: usize, x: f64, p: [f64; n]}

    let mut memo = vec![0.0; n + 1];
    memo[0] = 1.0;
    for &p in &p {
        let mut new = vec![0.0; n + 1];
        for i in (0..n + 1).rev() {
            new[i] += memo[i] * (100.0 - p) / 100.0;
            if i + 1 <= n {
                new[i + 1] += memo[i] * p / 100.0;
            }
        }
        memo = new;
    }

    let mut dp = vec![0.0; x as usize + 1];
    for i in (0..x as usize).rev() {
        let mut sum = 0.;
        for j in 1..n + 1 {
            sum += (dp.get(i + j).unwrap_or(&0.0) + 1.0) * memo[j];
        }
        sum += memo[0];
        sum /= 1.0 - memo[0];
        dp[i] = sum;
    }

    println!("{}", dp[0])
}
