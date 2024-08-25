use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, x: usize, p: [(usize, usize, usize); n-1]}

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    for i in 0..n {
        let mut dist = vec![std::usize::MAX; n];
        dist[i] = 0;
        let mut nt = BinaryHeap::new();
        nt.push(Reverse((0, i)));
        while let Some(Reverse((nd, now))) = nt.pop() {
            if dist[now] < nd {
                continue;
            }
            for &(to, c) in &t[now] {
                if dist[to] > nd + c {
                    dist[to] = nd + c;
                    nt.push(Reverse((nd + c, to)));
                }
            }
        }

        if dist.iter().any(|&d| d == x) {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
