use std::cmp::Reverse;

use proconio::*;

const MAX: usize = 1440;

fn main() {
    input! {n: usize, mut p: [(usize, usize); n]}
    p.sort_unstable_by_key(|v| (v.1, Reverse(v.0)));

    let mut dp = vec![0; MAX + 1];
    for (t, d) in p {
        for now in (0..MAX + 1).rev() {
            if now + t > d {
                continue;
            }

            dp[now + t] = dp[now + t].max(dp[now] + 1);
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
