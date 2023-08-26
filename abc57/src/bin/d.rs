use std::collections::HashMap;

use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    input! {n: usize, a: usize, b: usize, v: [usize; n]}

    let mut dp = vec![0; n + 1];
    let mut memo = vec![0; n + 1];
    memo[0] = 1usize;
    let mut reached = vec![false; n + 1];
    reached[0] = true;

    for v in v {
        for i in (0..n).rev() {
            if reached[i] {
                if dp[i + 1] < dp[i] + v {
                    dp[i + 1] = dp[i] + v;
                    memo[i + 1] = memo[i];
                    reached[i + 1] = true;
                } else if dp[i + 1] == dp[i] + v {
                    memo[i + 1] += memo[i];
                    reached[i + 1] = true;
                }
            }
        }
    }

    let mut map = HashMap::new();
    for i in a..=b {
        let d = dp[i];
        let m = memo[i];
        let g = gcd(d, i);
        *map.entry((d / g, i / g)).or_insert(0) += m;
    }

    let mut res = map
        .into_iter()
        .map(|((n, d), v)| (n as f64 / d as f64, v))
        .collect::<Vec<_>>();
    res.sort_by(|&l, &r| l.0.partial_cmp(&r.0).unwrap());

    let res = res.pop().unwrap();
    println!("{}", res.0);
    println!("{}", res.1)
}
