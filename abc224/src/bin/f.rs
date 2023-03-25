#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    const MOD: usize = 998244353;
    input! {s: Chars}
    let n = s.len();

    let mut dp = vec![0; n];
    let mut dp2 = vec![0; n];

    dp2[0] = s[0].to_digit(10).unwrap() as usize;

    let mut k = 1;
    for i in 1..n {
        // insert +
        dp[i] += dp[i-1] + dp2[i-1];
        dp2[i] += s[i].to_digit(10).unwrap() as usize * k;

        // not insert +
        dp[i] += dp[i-1];
        dp2[i] += dp2[i-1] * 10 + s[i].to_digit(10).unwrap() as usize * k;

        dp[i] %= MOD;
        dp2[i] %= MOD;
        k *= 2;
        k %= MOD;
    }

    println!("{}", (dp[n-1] + dp2[n-1]) % MOD);
}
