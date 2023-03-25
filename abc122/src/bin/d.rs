#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize}
    const MOD: usize = 1000000007;

    // AGCT = 0123
    let mut dp = vec![vec![vec![vec![0; 4]; 4]; 4]; n];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                if i == 0 && j == 1 && k == 2 {
                    continue;
                }
                if i == 0 && j == 2 && k == 1 {
                    continue;
                }
                if i == 1 && j == 0 && k == 2 {
                    continue;
                }
                dp[2][i][j][k] = 1
            }
        }
    }

    for i in 3..n {
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    for m in 0..4 {
                        if k == 1 && l == 0 && m == 2 {
                            continue;
                        }
                        if k == 0 && l == 1 && m == 2 {
                            continue;
                        }
                        if j == 0 && k == 1 && m == 2 {
                            continue;
                        }
                        if j == 0 && l == 1 && m == 2 {
                            continue;
                        }
                        if k == 0 && l == 2 && m == 1 {
                            continue;
                        }
                        dp[i][k][l][m] += dp[i-1][j][k][l];
                        dp[i][k][l][m] %= MOD;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                res += dp[n-1][i][j][k];
                res %= MOD;
            }
        }
    }

    println!("{}", res);
}
