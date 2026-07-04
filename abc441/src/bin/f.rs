use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, i64); n]}

    let mut dp = vec![-1i64; m + 1];
    dp[0] = 0;
    for &(p, v) in &p {
        for j in (0..m).rev() {
            if dp[j] >= 0 && j + p <= m {
                dp[j + p] = dp[j + p].max(dp[j] + v);
            }
        }
    }

    let mut memo = vec![vec![u32::MAX; m + 1]; n];
    let mut alt = vec![-1i64; m + 1];
    alt[0] = 0;
    for (i, &(p, v)) in p.iter().enumerate() {
        for j in (0..m).rev() {
            if alt[j] >= 0 && j + p <= m {
                alt[j + p] = alt[j + p].max(alt[j] + v);

                if alt[j] + v == dp[j + p] {
                    memo[i][j + p] = j as u32;
                }
            }
        }
    }

    let &max = dp.iter().max().unwrap();
    let mut reach = vec![0; m + 1];
    let mut ret = vec![];
    for i in (0..=m).rev() {
        if reach[i] > 0 || dp[i] == max {
            for j in 0..reach[i] {
                if memo[j][i] != u32::MAX {
                    reach[memo[j][i] as usize] = reach[memo[j][i] as usize].max(j);
                    ret.push(j);
                }
            }
        }
    }

    ret.sort_unstable();
    ret.dedup();

    println!("{}", ret.iter().map(|i| i + 1).join(" "))
}
