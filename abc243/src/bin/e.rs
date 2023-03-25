#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]}

    let mut dp = vec![vec![std::i64::MAX / 2; n]; n];
    let mut set = std::collections::HashSet::new();
    for (f, t, c) in p {
        dp[f-1][t-1] = c;
        dp[t-1][f-1] = c;
        set.insert((f-1, t-1));
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dp[i][k] + dp[k][j] <= dp[i][j] {
                    if set.contains(&(i, j)) {
                        set.remove(&(i, j));
                    } else if set.contains(&(j, i)) {
                        set.remove(&(j, i));
                    }
                    dp[i][j] = dp[i][k] + dp[k][j];
                }
            }
        }
    }

    println!("{}", m - set.len());
}
