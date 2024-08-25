use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, a: [usize; n]}

    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut min = usize::MAX;
        let mut max = 0;
        for j in (0..m).take_while(|&j| i + j < n) {
            min = min.min(a[i + j]);
            max = max.max(a[i + j]);
            dp[i + j + 1] = dp[i + j + 1].min(dp[i] + k + (j + 1) * (max - min));
        }
    }

    println!("{}", dp[n])
}
