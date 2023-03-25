#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: usize, b: usize, c: usize}

    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];

    for i in (0..100).rev() {
        for j in (0..100).rev() {
            for k in (0..100).rev() {
                dp[i][j][k] = (i as f64 * dp[i+1][j][k] + j as f64 * dp[i][j+1][k] + k as f64 * dp[i][j][k+1]) as f64 / (i+j+k) as f64 + 1.0;
            }
        }
    }

    println!("{}", dp[a][b][c]);
}
