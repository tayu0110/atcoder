use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); n]}

    let mut dp = vec![0; m + 1];
    for (v, t) in p {
        for now in (0..m).rev() {
            if now + t > m {
                continue;
            }

            dp[now + t] = dp[now + t].max(dp[now] + v);
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
