use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, i64); m]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in e {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut dist = vec![(usize::MAX, i64::MAX); n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0, 0)));
    while let Some(Reverse((cnt, nd, now))) = nt.pop() {
        if dist[now] <= (cnt, nd) {
            continue;
        }
        dist[now] = (cnt, nd);
        for &(to, w) in &t[now] {
            if dist[to] > (to + 1, nd + 1) {
                nt.push(Reverse((cnt + 1, nd + 1, to)));
            }
            if w > 0 && dist[to] > (cnt, nd + 1 + (w - nd).max(0)) {
                nt.push(Reverse((cnt, nd + 1 + (w - nd).max(0), to)));
            }
        }
    }
    println!("{}", dist[n - 1].1);
}
