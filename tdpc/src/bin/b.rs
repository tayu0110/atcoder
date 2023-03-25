#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, p: [i32; a], q: [i32; b]};

    let mut dp = vec![vec![0; b+1]; a+1];
    for i in 0..a+1 {
        for j in 0..b+1 {
            if (i + j) % 2 == 0 {
                if i < a {
                    dp[i+1][j] = std::cmp::max(dp[i+1][j], dp[i][j] + p[i]);
                }
                if j < b {
                    dp[i][j+1] = std::cmp::max(dp[i][j+1], dp[i][j] + q[j]);
                }
            } else {
                if i < a {
                    dp[i+1][j] = std::cmp::max(dp[i+1][j], dp[i][j]);
                }
                if j < b {
                    dp[i][j+1] = std::cmp::max(dp[i][j+1], dp[i][j]);
                }
            }
        }
    }

    let res = dp.iter().fold(0, |max, v| std::cmp::max(max, *v.iter().max().unwrap()));
    println!("{}", res);
}
