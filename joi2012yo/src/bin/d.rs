use proconio::*;

const M: usize = 10000;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); k]}

    let mut mask = vec![14; n];
    for (a, b) in p {
        mask[a - 1] = 1 << b;
    }

    let mut dp = vec![vec![vec![0; n + 1]; 4]; 4];
    dp[0][0][0] = 1;
    for k in 0..n {
        for i in 0..4 {
            for j in 0..4 {
                if dp[i][j][k] == 0 {
                    continue;
                }
                for next in 0..4 {
                    if i == j && j == next {
                        continue;
                    }

                    if mask[k] & (1 << next) == 0 {
                        continue;
                    }

                    dp[j][next][k + 1] += dp[i][j][k];
                    dp[j][next][k + 1] %= M;
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..4 {
        for j in 0..4 {
            res += dp[i][j][n];
            res %= M;
        }
    }

    println!("{res}")
}
