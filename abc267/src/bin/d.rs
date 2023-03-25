#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, a: [i64; n]};
    const INF: i64 = 0x3f3f3f3f3f3f3f3f;

    let mut dp = vec![-INF; m+1];
    dp[0] = 0;

    for i in 0..n {
        for j in (1..m+1).rev() {
            dp[j] = std::cmp::max(dp[j], dp[j-1] + a[i] * j as i64);
        }
    }

    println!("{}", dp[m]);
}
