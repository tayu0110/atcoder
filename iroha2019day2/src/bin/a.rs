#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars, t: Chars};
    let n = s.len();

    let mut dp = vec![vec![0; n+1]; n+1];
    for i in 1..=n {
        for j in 1..=n {
            if s[i-1] == t[j-1] {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i-1][j-1] + 1);
            } else {
                dp[i][j] = std::cmp::max(dp[i][j], std::cmp::max(dp[i-1][j], dp[i][j-1]));
            }
        }
    }

    let max = dp.iter().fold(0, |s, v| std::cmp::max(s, *v.iter().max().unwrap()));
    println!("{}", max + 1);
}
