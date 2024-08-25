use proconio::*;

fn main() {
    input! {n: usize, x: usize, y: usize, p: [(usize, usize); n]}

    let mut dp = vec![vec![usize::MAX; x + 1]; n + 1];
    dp[0][0] = 0;
    for (a, b) in p {
        for i in (0..n).rev() {
            for j in 0..x {
                if dp[i][j] == usize::MAX {
                    continue;
                }

                if j + a > x {
                    continue;
                }

                dp[i + 1][j + a] = dp[i + 1][j + a].min(dp[i][j] + b);
            }
        }
    }

    if dp[n].iter().any(|v| *v <= y) {
        println!("{n}");
        return;
    }

    for i in (0..n).rev() {
        if dp[i].iter().any(|&v| v <= y) {
            println!("{}", i + 1);
            return;
        }
    }
}
