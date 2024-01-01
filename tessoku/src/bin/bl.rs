use itertools::Itertools;
use proconio::*;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut dist = vec![usize::MAX; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if dist[now] < usize::MAX {
            continue;
        }
        dist[now] = nd;

        for &(to, c) in &t[now] {
            if dist[to] == usize::MAX {
                nt.push(Reverse((nd + c, to)));
            }
        }
    }

    println!("{}", dist.iter().map(|v| *v as i64).join("\n"))
}
