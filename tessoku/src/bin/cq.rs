use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, s: usize, a: [usize; n]}

    let mut dp = vec![usize::MAX; s + 1];
    dp[0] = 0;
    for (i, &a) in a.iter().enumerate() {
        for j in (0..s + 1).rev() {
            if dp[j] < usize::MAX && j + a <= s && dp[j + a] == usize::MAX {
                dp[j + a] = i + 1;
            }
        }
    }

    if dp[s] == usize::MAX {
        println!("-1")
    } else {
        let mut now = s;
        let mut res = vec![];
        while dp[now] > 0 {
            res.push(dp[now]);
            now -= a[dp[now] - 1];
        }

        println!("{}", res.len());
        println!("{}", res.iter().join(" "))
    }
}
