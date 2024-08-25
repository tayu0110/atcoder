use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize, c: usize, d: [usize; n]}

    let mut dp = vec![0; n + 1];
    dp[0] = c;
    for d in d {
        for j in (0..n).rev() {
            dp[j + 1] = dp[j + 1].max(dp[j] + d);
        }
    }

    let mut now = a;
    let (mut s, mut t) = (0, 1);
    for i in 0..=n {
        if dp[i] * t > s * now {
            (s, t) = (dp[i], now);
        }
        now += b;
    }

    println!("{}", s / t);
}
