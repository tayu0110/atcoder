#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, k: usize, a: [usize; n]};

    if k == a.iter().sum() {
        println!("1");
        std::process::exit(0);
    }

    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let mut dp = vec![vec![INF; n+1]; n+1];
    dp[0][0] = 0;
    
    let mut sum = 0;
    for i in 0..n {
        for j in 0..n {
            if dp[i][j] == INF {
                continue;
            }
            dp[i+1][j] = std::cmp::min(dp[i+1][j], dp[i][j]);
            if sum == 0 {
                dp[i+1][j+1] = std::cmp::min(dp[i+1][j+1], dp[i][j] + 1);
            } else {
                let new = (dp[i][j] * a[i]) / sum + 1;
                if new > a[i] {
                    continue;
                }
                dp[i+1][j+1] = std::cmp::min(dp[i+1][j+1], dp[i][j] + new);
            }
        }
        sum += a[i];
    }

    for i in (0..n+1).rev() {
        if dp[n][i] <= k {
            println!("{}", i);
            std::process::exit(0);
        }
    }
}
