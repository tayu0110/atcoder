use std::collections::HashSet;

use itertools::Itertools;
use proconio::*;
// use rand::{thread_rng, Rng};

fn main() {
    input! {n: usize, a: [i64; n]}
    // let n = 80;
    // let mut rng = thread_rng();
    // let a = (0..n).map(|_| rng.gen_range(1..=100)).collect::<Vec<i64>>();

    let mut set = HashSet::new();
    let mut res = vec![0; n + 1];
    for i in 0..n {
        for j in i + 1..n {
            let diff = a[j] - a[i];
            if !set.contains(&diff) {
                set.insert(diff);

                let mut dp = vec![vec![0; n]; n + 1];
                for k in 0..n {
                    dp[1][k] = 1;
                    for l in 0..k {
                        if a[k] - a[l] == diff {
                            for m in 1..=k {
                                dp[m + 1][k] += dp[m][l];
                                dp[m + 1][k] %= 998244353;
                            }
                        }
                    }
                }

                for k in 1..=n {
                    res[k] = dp[k].iter().fold(res[k], |s, v| (s + v) % 998244353);
                }
            }
        }
    }

    res[1] = n;
    println!("{}", res[1..].iter().join(" "))
}
