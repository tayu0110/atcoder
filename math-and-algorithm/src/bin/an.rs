use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut nt = VecDeque::new();
    nt.push_back(0);
    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[to] == usize::MAX {
                dist[to] = dist[now] + 1;
                nt.push_back(to);
            }
        }
    }

    println!("{}", dist.into_iter().map(|d| d as i64).join("\n"))
}
