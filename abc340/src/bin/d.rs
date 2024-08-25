use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize, usize); n - 1]}

    let mut t = vec![vec![]; n + 1];
    for (i, (a, b, x)) in e.into_iter().enumerate() {
        t[i + 1].push((i + 2, a));
        t[i + 1].push((x, b));
    }

    let mut d = vec![usize::MAX; n + 1];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 1)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if d[now] != usize::MAX {
            continue;
        }
        d[now] = nd;
        for &(to, w) in &t[now] {
            if d[to] == usize::MAX {
                nt.push(Reverse((nd + w, to)));
            }
        }
    }

    println!("{}", d[n])
}
