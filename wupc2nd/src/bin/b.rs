use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut dp = vec![usize::MAX; n];
    dp[0] = 0;

    for i in 0..n {
        for j in 1..=3 {
            if i + j < n {
                dp[i + j] = dp[i + j].min(dp[i] + (s[i] == b'X') as usize);
            }
        }
    }

    println!("{}", dp[n - 1])
}
