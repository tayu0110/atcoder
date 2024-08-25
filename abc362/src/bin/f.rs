use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for &(u, v) in &e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut root = 0;
    let mut dist = vec![usize::MAX; n];
    for _ in 0..2 {
        dist.fill(usize::MAX);
        let mut nt = VecDeque::new();
        nt.push_back(root);
        dist[root] = 0;
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if dist[to] == usize::MAX {
                    dist[to] = dist[now] + 1;
                    nt.push_back(to);
                }
            }
        }

        let &max = dist.iter().max().unwrap();
        root = dist.iter().position(|&p| p == max).unwrap();
    }

    let &max = dist.iter().max().unwrap();
    let half = max / 2;
    let mid = dist.iter().position(|&p| p == half).unwrap();
}
