use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push((v - 1, 0));
        t[v - 1].push((u - 1, 1));
    }

    let mut dist = vec![vec![usize::MAX; n]; 2];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0, 0)));
    while let Some(Reverse((nd, now, direction))) = nt.pop() {
        if dist[direction][now] != usize::MAX {
            continue;
        }
        dist[direction][now] = nd;
        for &(to, dir) in &t[now] {
            if dir == direction {
                if dist[dir][to] == usize::MAX {
                    nt.push(Reverse((nd + 1, to, dir)));
                }
            } else {
                if dist[dir][to] == usize::MAX {
                    nt.push(Reverse((nd + x + 1, to, dir)));
                }
            }
        }
    }

    println!("{}", dist[0][n - 1].min(dist[1][n - 1]));
}
