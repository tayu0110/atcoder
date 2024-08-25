use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort_unstable();

    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for i in 0..n {
        if i + 2 <= n {
            dp[i + 2] = dp[i + 2].min(dp[i].saturating_add(a[i + 1]) - a[i]);
        }
        if i + 3 <= n {
            dp[i + 3] = dp[i + 3].min(dp[i].saturating_add(a[i + 2]) - a[i]);
        }
    }

    println!("{}", dp[n])
}
