use proconio::*;

const M: usize = 1001;

fn main() {
    input! {n: usize, m: usize, d: [u32; n], c: [u32; m]}

    let mut dp = [0; M];
    let mut next = [u32::MAX; M];
    for i in 0..n {
        for j in 0..m {
            if dp[j] < u32::MAX {
                next[j + 1] = next[j + 1].min(dp[j] + d[i] * c[j]);
                dp[j + 1] = dp[j + 1].min(dp[j]);
            }
        }
        (dp, next) = (next, dp);
        next.fill(u32::MAX);
    }

    println!("{}", dp.iter().min().unwrap());
}
