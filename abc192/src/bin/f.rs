#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: i64, a: [i64; n]}

    let mut res = std::i64::MAX;
    for k in 1..=n {
        let rx = x % k as i64;
        let mut dp = vec![vec![vec![-1; k]; k+1]; n+1];
        dp[0][0][0] = 0;

        let a = a.clone();
        for (l, v) in a.into_iter().enumerate() {
            for i in 0..=k {
                for j in 0..k {
                    dp[l+1][i][j] = std::cmp::max(dp[l+1][i][j], dp[l][i][j]);
                    if i < k && dp[l][i][j] >= 0 {
                        dp[l+1][i+1][(j + v as usize) % k] = std::cmp::max(dp[l+1][i+1][(j + v as usize) % k], dp[l][i][j] + v);
                    }
                }
            }
        }

        // eprintln!("k: {}, rx: {}, dp[k][rx]: {}", k, rx, dp[n][k][rx as usize]);

        if dp[n][k][rx as usize] < 0 {
            continue;
        }

        res = std::cmp::min(res, (x - dp[n][k][rx as usize]) / k as i64);
    }

    println!("{}", res);
}
