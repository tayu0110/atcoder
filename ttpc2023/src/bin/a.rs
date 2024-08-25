use proconio::*;

fn main() {
    input! {n: usize}

    let mut dp = vec![0; n + 1];
    for i in 2..=n {
        dp[i] = dp[i - 1] * 2 + i - 1;
        dp[i] %= 998244353;
    }

    println!("{}", dp[n])
}
