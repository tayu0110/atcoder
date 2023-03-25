#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(i64, i64, i64); n]}
    const INF: i64 = 0x3f3f3f3f3f3f3f3f;

    let mut dp = vec![vec![INF; 1 << n]; n];
    dp[0][1] = 0;
    for i in 1..(1 << n) {
        for j in 0..n {
            if i & (1 << j) == 0 {
                continue;
            }
            for k in 0..n {
                let to = i | (1 << k);
                let cost = (p[k].0 - p[j].0).abs() + (p[k].1 - p[j].1).abs() + std::cmp::max(0, p[k].2 - p[j].2);
                dp[k][to] = std::cmp::min(dp[k][to], dp[j][i] + cost);
            }
        }
    }

    println!("{}", dp[0][(1 << n) - 1]);
}
