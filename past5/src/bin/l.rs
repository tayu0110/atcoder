use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes, t: marker::Bytes}

    let mut dp = vec![vec![-1; n + 1]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for len in 3..=n {
        for l in 0..n {
            let r = l + len;
            if r > n {
                continue;
            }
            for mid in l + 1..r - 1 {
                if &t[..] == &[s[l], s[mid], s[r - 1]]
                    && dp[l + 1][mid] >= 0
                    && dp[mid + 1][r - 1] >= 0
                {
                    dp[l][r] = dp[l][r].max(dp[l + 1][mid] + dp[mid + 1][r - 1] + 1);
                }
            }
            for mid in l..r {
                if dp[l][mid] >= 0 && dp[mid][r] >= 0 {
                    dp[l][r] = dp[l][r].max(dp[l][mid] + dp[mid][r]);
                }
            }
        }
    }

    let mut ndp = vec![0; n + 1];
    for i in 0..n {
        for j in i..n + 1 {
            ndp[j] = ndp[j].max(ndp[i] + dp[i][j].max(0));
        }
    }

    println!("{}", ndp[n])
}
