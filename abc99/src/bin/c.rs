#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize}

    let mut dp = vec![std::usize::MAX; n + 1];
    dp[0] = 0;
    for i in 0..n {
        dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + 1);

        let mut six = 6;
        while i + six <= n {
            dp[i + six] = std::cmp::min(dp[i + six], dp[i] + 1);
            six *= 6;
        }

        let mut nine = 9;
        while i + nine <= n {
            dp[i + nine] = std::cmp::min(dp[i + nine], dp[i] + 1);
            nine *= 9;
        }
    }

    println!("{}", dp[n]);
}
