#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    const MOD: usize = 998244353;
    const CHARS: usize = 10;
    input! {n: usize, s: Bytes}
    let s = s.into_iter().map(|c| (c - b'A') as usize).collect::<Vec<_>>();

    let mut dp = vec![vec![vec![0; n+1]; CHARS]; 1 << CHARS];
    dp[0][0][0] = 1;

    for k in 0..n {
        for i in 0..(1 << CHARS) {
            for j in 0..CHARS {
                dp[i][j][k+1] += dp[i][j][k];
                dp[i][j][k+1] %= MOD;
                if dp[i][j][k] == 0 {
                    continue;
                }

                if j != s[k] && i & (1 << s[k]) != 0 {
                    continue;
                }

                dp[i | (1 << s[k])][s[k]][k+1] += dp[i][j][k];
                dp[i | (1 << s[k])][s[k]][k+1] %= MOD;
            }
        }
    }

    let mut res = 0;
    for i in 1..(1 << CHARS) {
        for j in 0..CHARS {
            res += dp[i][j][n];
            res %= MOD;
        }
    }

    println!("{}", res);
}
