#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize, i64); n]};

    let mut t = 0;
    let mut map = std::collections::HashMap::new();
    for (nt, x, a) in p {
        t = std::cmp::max(t, nt);
        map.entry(nt).or_insert([0; 5])[x] += a;
    }

    let mut dp = vec![vec![-1; 5]; t+2];
    dp[0][0] = 0;
    for i in 0..=t {
        for j in 0..5 {
            if dp[i][j] < 0 {
                continue;
            }
            if let Some(v) = map.get(&(i+1)) {
                dp[i+1][j] = std::cmp::max(dp[i+1][j], dp[i][j] + v[j]);
                if j > 0 {
                    dp[i+1][j-1] = std::cmp::max(dp[i+1][j-1], dp[i][j] + v[j-1]);
                }
                if j + 1 < 5 {
                    dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j] + v[j+1]);
                }
            } else {
                dp[i+1][j] = std::cmp::max(dp[i+1][j], dp[i][j]);
                if j > 0 {
                    dp[i+1][j-1] = std::cmp::max(dp[i+1][j-1], dp[i][j]);
                }
                if j + 1 < 5 {
                    dp[i+1][j+1] = std::cmp::max(dp[i+1][j+1], dp[i][j]);
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..5 {
        res = std::cmp::max(res, dp[t+1][i]);
    }
    println!("{}", res);
}
