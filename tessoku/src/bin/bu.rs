use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b, c, d) in e {
        t[a - 1].push((b - 1, c, d));
        t[b - 1].push((a - 1, c, d));
    }

    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, m, 0)));
    let mut dist = vec![usize::MAX; n];
    let mut cnt = vec![0; n];
    while let Some(Reverse((nd, tree, now))) = nt.pop() {
        if dist[now] != usize::MAX {
            continue;
        }
        dist[now] = nd;
        cnt[now] = tree;

        for &(to, c, d) in &t[now] {
            if dist[to] == usize::MAX {
                nt.push(Reverse((nd + c, tree - d, to)));
            }
        }
    }

    println!("{} {}", dist[n - 1], m - cnt[n - 1])
}
