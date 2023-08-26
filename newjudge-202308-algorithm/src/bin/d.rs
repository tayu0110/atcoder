use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n1: usize, n2: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n1 + n2];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut dist = vec![usize::MAX; n1 + n2];
    let mut nt = VecDeque::new();
    nt.push_back(0);
    dist[0] = 0;
    nt.push_back(n1 + n2 - 1);
    dist[n1 + n2 - 1] = 0;

    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[to] == usize::MAX {
                dist[to] = dist[now] + 1;
                nt.push_back(to);
            }
        }
    }

    println!(
        "{}",
        dist[..n1].iter().max().unwrap() + dist[n1..].iter().max().unwrap() + 1
    )
}
