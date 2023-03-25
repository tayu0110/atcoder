#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize); n]}

    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for (a, b) in p {
        for _ in 0..b {
            for i in (0..=x).rev() {
                if !dp[i] {
                    continue;
                }

                if i + a > x {
                    continue;
                }

                dp[i + a] = true;
            }
        }
    }

    if dp[x] {
        println!("Yes")
    } else {
        println!("No")
    }
}
