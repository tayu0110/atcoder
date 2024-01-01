use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}
    a.sort();

    let mut dp = vec![vec![usize::MAX; 35]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..34 {
            if dp[i][j] < usize::MAX {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + 1);
                let (mut l, mut r) = (-1, n as i32);
                while r - l > 1 {
                    let m = (r + l) / 2;
                    if a[i] * 2 <= a[m as usize] {
                        r = m;
                    } else {
                        l = m;
                    }
                }

                dp[r as usize][j + 1] = dp[r as usize][j + 1].min(dp[i][j]);
            }
        }
    }

    for i in 0..35 {
        if dp[n][i] <= k {
            println!("{i} {}", dp[n][i]);
            return;
        }
    }
}
