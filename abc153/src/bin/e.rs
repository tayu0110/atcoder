#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {h: usize, n: usize, p: [(usize, usize); n]}

    let mut dp = vec![std::usize::MAX; h + p.iter().map(|(a, _)| *a).max().unwrap() + 1];
    dp[0] = 0;

    for i in 0..h {
        if dp[i] == std::usize::MAX {
            continue;
        }

        for &(a, b) in &p {
            dp[i+a] = std::cmp::min(dp[i+a], dp[i]+b);
        }
    }

    println!("{}", dp.iter().skip(h).min().unwrap());
}
