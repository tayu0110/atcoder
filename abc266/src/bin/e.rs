#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};

    let mut dp = vec![0f64; n+1];
    dp[1] = 3.5;
    for i in 2..=n {
        let retry = dp[i-1].floor() as usize;
        let f = (retry+1..=6).sum::<usize>();
        dp[i] = dp[i-1] * retry as f64 / 6.0 + f as f64 / 6.0;
    }

    println!("{}", dp[n]);
}
