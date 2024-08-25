use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, k: usize, s: marker::Bytes}

    let mut dp = vec![vec![0; 1 << k]; n + 1];
    dp[0][0] = 1;

    let mut bad = vec![false; 1 << k];
    for i in 0..1 << k {
        if (0..k).all(|j| (i >> j) & 1 == (i >> (k - 1 - j)) & 1) {
            bad[i] = true;
        }
    }

    let mask = (1 << k) - 1;
    for (i, s) in s.into_iter().enumerate() {
        for j in 0..1 << k {
            if dp[i][j] == 0 {
                continue;
            }

            if s == b'A' || s == b'B' {
                let next = ((j << 1) | (s - b'A') as usize) & mask;
                if !bad[next] || i < k - 1 {
                    dp[i + 1][next] += dp[i][j];
                    dp[i + 1][next] %= M;
                }
            } else {
                for b in 0..2 {
                    let next = ((j << 1) | b) & mask;
                    if !bad[next] || i < k - 1 {
                        dp[i + 1][next] += dp[i][j];
                        dp[i + 1][next] %= M;
                    }
                }
            }
        }
    }

    println!("{}", dp[n].iter().fold(0, |s, v| (s + v) % M))
}
