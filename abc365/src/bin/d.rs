use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    let s = s
        .iter()
        .map(|&s| {
            if s == 'R' {
                0
            } else if s == 'P' {
                1
            } else {
                2
            }
        })
        .collect::<Vec<_>>();
    let mut dp = vec![vec![-1; 3]; n + 1];
    dp[0] = vec![0, 0, 0];
    for (i, s) in s.into_iter().enumerate() {
        for j in 0..3 {
            if (s == 0 && j == 2) || (s == 1 && j == 0) || (s == 2 && j == 1) {
                continue;
            }

            for k in 0..3 {
                if dp[i][k] < 0 {
                    continue;
                }
                if j == k {
                    continue;
                }

                let win = (s == 0 && j == 1) || (s == 1 && j == 2) || (s == 2 && j == 0);
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][k] + win as i32);
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
