#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, s: [String; n]}

    let mut dp = vec![vec![0usize; 2]; n+1];
    dp[0][0] = 1;
    dp[0][1] = 1;

    for (i, op) in s.into_iter().enumerate() {
        for j in 0..2 {
            for k in 0..2 {
                let to = if op == "AND" {
                    j & k
                } else {
                    j | k
                };
                dp[i+1][to] += dp[i][j];
            }
        }
    }

    println!("{}", dp[n][1]);
}
