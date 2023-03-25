#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    const MOD: i64 = 998244353;
    input! {n: usize, a: [i64; n], b: [i64; n]}

    let mut dp = vec![vec![0i64; 3010]; n+1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..=3000 {
            let min = std::cmp::max(j, a[i]);
            let max = std::cmp::max(j, b[i]+1);

            dp[i+1][min as usize] += dp[i][j as usize];
            dp[i+1][max as usize] -= dp[i][j as usize];
        }

        for j in 0..=3000 {
            dp[i+1][j+1] += dp[i+1][j];
            dp[i+1][j+1] %= MOD;
        }
    }

    println!("{}", dp[n].iter().fold(0, |s, v| (s + *v) % MOD));
}
