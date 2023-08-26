use proconio::*;

const MIN: i32 = std::i32::MIN >> 5;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut dp = vec![vec![MIN; n + 1]; n + 1];
    for i in 0..=n {
        dp[i][i] = 0;
    }

    for len in 1..=n {
        for l in (0..=n).take_while(|l| l + len <= n) {
            let r = l + len;
            for m in l + 1..r {
                dp[l][r] = dp[l][r].max(dp[l][m] + dp[m][r]);
            }
            for m in l + 1..r - 1 {
                dp[l][r] =
                    dp[l][r].max(dp[l + 1][m] + dp[m + 1][r - 1] + (a[l] + a[m] + a[r - 1]).max(0));
            }
        }
    }

    let mut res = 0;
    if n % 3 == 0 {
        res = res.max(dp[0][n]);
    } else if n % 3 == 1 {
        for i in 0..n {
            res = res.max(dp[0][i] + dp[i + 1][n]);
        }
    } else {
        for i in 0..n {
            for j in i + 1..n {
                res = res.max(dp[0][i] + dp[i + 1][j] + dp[j + 1][n]);
            }
        }
    }

    println!("{}", res)
}
