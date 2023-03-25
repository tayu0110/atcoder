#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};


fn main() {
    input! {mut n: usize, k: usize, a: [usize; k]}

    let mut dp = vec![0; n+1];
    for i in 0..=n {
        for j in (0..k).take_while(|j| a[*j] <= i) {
            dp[i] = std::cmp::max(dp[i], a[j] + (i - a[j]) - dp[i - a[j]]);
        }
    }

    println!("{}", dp[n]);
}
