use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], mut s: usize, mut t: usize}
    s -= 1;
    t -= 1;

    let mut g = vec![vec![]; n];
    for (u, v) in e {
        g[u - 1].push(v - 1);
    }

    let mut dist = vec![vec![usize::MAX; 3]; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, s)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if dist[now][nd % 3] < usize::MAX {
            continue;
        }
        dist[now][nd % 3] = nd;
        for &to in &g[now] {
            let next = nd + 1;
            if dist[to][next % 3] == usize::MAX {
                nt.push(Reverse((next, to)));
            }
        }
    }

    if dist[t][0] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[t][0] / 3);
    }
}
