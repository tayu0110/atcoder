use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n * (n - 1)]}

    let mut read = a.into_iter();
    let mut a = vec![vec![vec![0; n]; n]; 2];
    for i in 0..2 {
        for j in 0..n {
            for k in j + 1..n {
                a[i][j][k] = read.next().unwrap();
                a[i][k][j] = a[i][j][k];
            }
        }
    }

    let mut dp = a[0].clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }
    let mut new = dp.clone();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                new[i][j] = new[i][j].min(dp[i][k] + dp[k][j].min(a[1][k][j]));
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                new[i][j] = new[i][j].min(dp[i][k].min(a[1][i][k]) + dp[k][j]);
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                new[i][j] = new[i][j].min(new[i][k] + dp[k][j]);
            }
        }
    }

    println!(
        "{}",
        (0..n - 1)
            .map(|i| (i + 1..n).map(|j| new[i][j].min(new[j][i])).join(" "))
            .join("\n")
    )
}
