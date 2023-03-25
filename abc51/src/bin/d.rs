use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (i, (a, b, c)) in p.into_iter().enumerate() {
        t[a - 1].push((b - 1, c, i));
        t[b - 1].push((a - 1, c, i));
    }

    let mut used = vec![false; m];
    for now in 0..n {
        let mut nt = BinaryHeap::new();
        nt.push(Reverse((0, now, std::usize::MAX)));

        let mut dist = vec![std::usize::MAX; n];

        while let Some(Reverse((nd, now, e))) = nt.pop() {
            if nd <= dist[now] && e != std::usize::MAX {
                used[e] = true;
            }
            if nd > dist[now] {
                continue;
            }
            dist[now] = nd;

            for &(to, c, ne) in &t[now] {
                if dist[now] + c <= dist[to] {
                    nt.push(Reverse((dist[now] + c, to, ne)));
                }
            }
        }
    }

    println!("{}", used.into_iter().filter(|&v| !v).count())
}
