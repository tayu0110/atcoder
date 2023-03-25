use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut memo = vec![vec![1; 10]; m + 1];
    for v in s.windows(2) {
        let mut dp = vec![vec![vec![vec![0; 2]; 10]; 10]; m + 1];
        dp[0][0][0][1] = 1;
        if let [s0, s1] = v {
            for (i, (&c0, &c1)) in s0.into_iter().zip(s1.into_iter()).enumerate() {
                for j in 0..10 {
                    for k in 0..10 {
                        for f in 0..2 {
                            if c0 != '?' && c1 != '?' {
                                if c0 > c1 && f == 1 {
                                    continue;
                                }

                                let (c0, c1) =
                                    (c0 as usize - b'0' as usize, c1 as usize - b'0' as usize);

                                dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] +=
                                    dp[i][j][k][f] * memo[i + 1][c1];
                                dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] %= MOD;
                            } else if c0 == '?' && c1 == '?' {
                                for c0 in 0..10 {
                                    for c1 in 0..10 {
                                        if c0 > c1 && f == 1 {
                                            continue;
                                        }

                                        dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] +=
                                            dp[i][j][k][f] * memo[i + 1][c1];
                                        dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] %= MOD;
                                    }
                                }
                            } else if c0 != '?' {
                                let c0 = c0 as usize - b'0' as usize;
                                for c1 in 0..10 {
                                    if c0 > c1 && f == 1 {
                                        continue;
                                    }

                                    dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] +=
                                        dp[i][j][k][f] * memo[i + 1][c1];
                                    dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] %= MOD;
                                }
                            } else {
                                let c1 = c1 as usize - b'0' as usize;
                                for c0 in 0..10 {
                                    if c0 > c1 && f == 1 {
                                        continue;
                                    }

                                    dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] +=
                                        dp[i][j][k][f] * memo[i + 1][c1];
                                    dp[i + 1][c0][c1][(f != 0 && c0 == c1) as usize] %= MOD;
                                }
                            }
                        }
                    }
                }
            }

            let mut new = vec![vec![0; 10]; m + 1];
            for i in 0..=m {
                for j in 0..10 {
                    for k in 0..10 {
                        new[i][k] += dp[i][j][k][0] + dp[i][j][k][1];
                        new[i][k] %= MOD;
                    }
                }
            }
            memo = new;
        }
    }

    let mut res = 0;
    for i in 0..10 {
        res += memo[m][i];
        res %= MOD;
    }

    println!("{}", res);
}
