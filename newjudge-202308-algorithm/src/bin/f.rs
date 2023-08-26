use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {s: usize, t: usize, m: usize, e: [(usize, usize); m]}

    let mut g = vec![vec![]; s + t];
    for (u, v) in e {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut set = vec![HashSet::new(); s];
    for i in s..s + t {
        let mut memo = vec![vec![]; t];
        for &to in &g[i] {
            for &p in &g[to] {
                if p == i {
                    continue;
                }
                if !memo[p - s].is_empty() {
                    println!("{} {} {} {}", p + 1, i + 1, to + 1, memo[p - s][0] + 1);
                    return;
                }

                memo[p - s].push(to);
                set[to].insert(i);
            }
        }
    }

    println!("-1")
}
