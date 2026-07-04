use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n + 1];
    for (i, &a) in a.iter().enumerate() {
        t[i].push((i + 1, a));
        t[i + 1].push((i, 0));
    }
    for (l, r, c) in p {
        t[l - 1].push((r, c));
    }

    let mut dist = vec![usize::MAX; n + 1];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if dist[now] < usize::MAX {
            continue;
        }
        dist[now] = nd;
        for &(to, w) in &t[now] {
            if dist[to] == usize::MAX {
                nt.push(Reverse((nd + w, to)));
            }
        }
    }
    println!("{}", a.iter().sum::<usize>() - dist[n])
}
