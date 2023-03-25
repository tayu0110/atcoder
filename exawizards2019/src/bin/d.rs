use proconio::*;
use std::collections::HashMap;

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize, x: usize, mut s: [usize; n]}
    s.sort_by_key(|v| std::cmp::Reverse(*v));

    let mut dp = vec![HashMap::new(); n];
    for i in 0..n {
        dp[i].insert(x % s[i], 1);
    }

    for k in 0..n - 1 {
        let mut new = vec![HashMap::new(); n];

        for j in k..n {
            for (&nx, &v) in &dp[j] {
                for i in j + 1..n {
                    let entry = new[i].entry(nx % s[i]).or_insert(0);
                    *entry += v;
                    *entry %= MOD;
                }

                if k < j {
                    let entry = new[j].entry(nx).or_insert(0);
                    *entry += v * (j - k);
                    *entry %= MOD;
                }
            }
        }

        dp = new;
    }

    let mut res = 0;
    for (&x, &v) in &dp[n - 1] {
        res += v * x;
        res %= MOD;
    }

    println!("{}", res)
}
