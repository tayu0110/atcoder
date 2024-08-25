use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], e: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v, b) in e {
        t[u - 1].push((v - 1, b));
        t[v - 1].push((u - 1, b));
    }

    let mut dist = vec![usize::MAX; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((a[0], 0)));
    while let Some(Reverse((d, now))) = nt.pop() {
        if dist[now] < usize::MAX {
            continue;
        }

        dist[now] = d;

        for &(to, c) in &t[now] {
            if dist[to] == usize::MAX {
                nt.push(Reverse((d + c + a[to], to)));
            }
        }
    }

    println!("{}", dist[1..].iter().join(" "))
}
