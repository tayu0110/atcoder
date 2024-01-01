use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut t = vec![vec![]; n];
    for _ in 0..m {
        input! {a: usize, b: usize, c: u32}
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    let mut dist = vec![u32::MAX; n];
    while let Some(Reverse((nd, now))) = nt.pop() {
        if dist[now] < u32::MAX {
            continue;
        }
        dist[now] = nd;

        for &(to, c) in &t[now] {
            if dist[to] == u32::MAX {
                nt.push(Reverse((nd + c, to)));
            }
        }
    }

    let mut res = vec![n];
    let mut now = n - 1;
    while now != 0 {
        for &(to, c) in &t[now] {
            if dist[to] + c == dist[now] {
                now = to;
                res.push(to + 1);
                break;
            }
        }
    }

    println!("{}", res.iter().rev().join(" "))
}
