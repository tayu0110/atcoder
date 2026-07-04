use proconio::*;

fn main() {
    input! {n: usize, c: usize, a: [usize; n]}

    let mut dp = vec![vec![usize::MAX; n + 1]; n + 1];
    for i in 0..n {
        dp[i][i + 1] = c;
    }

    for len in 2..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                continue;
            }
            for mid in l + 1..r {
                dp[l][r] = dp[l][r].min(dp[l][mid] + dp[mid][r] + a[mid].abs_diff(a[mid - 1]) - c);
                dp[l][r] = dp[l][r].min(dp[l][mid] + dp[mid][r]);
            }
        }
    }
    println!("{}", dp[0][n]);
}
