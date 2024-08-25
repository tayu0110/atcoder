use proconio::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {_: usize, k: i32, s: marker::Chars}
    let max = k as usize * 2 + 1;

    let mut dp = vec![vec![0; max + 1]; max + 1];
    let f = |d: i32| {
        if d.abs() > k {
            None
        } else {
            Some((d + k) as usize)
        }
    };
    let g = |i: usize| i as i32 - k;
    dp[k as usize][k as usize] = 1;
    for c in s {
        let mut new = vec![vec![0; max + 1]; max + 1];
        for i in 0..=max {
            for j in 0..=max {
                if dp[i][j] == 0 {
                    continue;
                }
                let (ti, tj) = (g(i), g(j));
                let v = if c == '0' {
                    vec![((ti - 1).max(-1), (tj - 1).min(-1))]
                } else if c == '1' {
                    vec![((ti + 1).max(1), (tj + 1).min(1))]
                } else {
                    vec![
                        ((ti - 1).max(-1), (tj - 1).min(-1)),
                        ((ti + 1).max(1), (tj + 1).min(1)),
                    ]
                };

                for (ni, nj) in v {
                    if let (Some(ni), Some(nj)) = (f(ni), f(nj)) {
                        new[ni][nj] += dp[i][j];
                        new[ni][nj] %= MOD;
                    }
                }
            }
        }

        dp = new;
    }

    println!("{}", dp.iter().flatten().fold(0, |s, v| (s + v) % MOD))
}
