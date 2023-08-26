use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let n = s.len();

    let mut dp = vec![vec![0; n]; n];
    for len in 3..=n {
        for l in 0..n {
            let r = l + len - 1;
            if r >= n {
                break;
            }
            for mid in l..r {
                dp[l][r] = dp[l][r].max(dp[l][mid] + dp[mid + 1][r]);
                if s[l] == 'i'
                    && s[mid] == 'w'
                    && s[r] == 'i'
                    && 3 * (dp[l + 1][mid - 1] + dp[mid + 1][r - 1]) == len - 3
                {
                    dp[l][r] = dp[l][r].max(dp[l + 1][mid - 1] + dp[mid + 1][r - 1] + 1);
                }
            }
        }
    }

    println!("{}", dp[0][n - 1])
}
