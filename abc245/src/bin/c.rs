#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, k: usize, a: [[usize; n]; 2]}

    let mut dp = vec![vec![0; n]; 2];
    dp[0][0] = 1;
    dp[1][0] = 1;
    
    for i in 1..n {
        for j in 0..2 {
            let now = a[j][i];
            if std::cmp::max(now, a[0][i-1]) - std::cmp::min(now, a[0][i-1]) <= k {
                dp[j][i] |= dp[0][i-1];
            }
            if std::cmp::max(now, a[1][i-1]) - std::cmp::min(now, a[1][i-1]) <= k {
                dp[j][i] |= dp[1][i-1];
            }
        }
    }

    if dp[0][n-1] != 0 || dp[1][n-1] != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
