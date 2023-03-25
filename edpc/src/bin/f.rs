use proconio::{input, marker::Chars};

fn main() {
    input! {s: Chars, t: Chars}

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j - 1] + 1);
            } else {
                dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    let (mut i, mut j) = (s.len(), t.len());
    let mut res = vec![];
    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            if dp[i][j] > dp[i - 1][j - 1] {
                res.push(s[i - 1]);
            }
            i -= 1;
            j -= 1;
        }
    }

    res.reverse();
    println!("{}", res.into_iter().collect::<String>())
}
