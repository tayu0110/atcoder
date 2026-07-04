use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, y: usize, e: [(usize, usize, usize); m], x: [usize; n]}

    let mut t = vec![vec![]; n + 1];
    for (u, v, tm) in e {
        t[u - 1].push((v - 1, tm));
        t[v - 1].push((u - 1, tm));
    }

    for (i, x) in x.into_iter().enumerate() {
        t[i].push((n, x));
        t[n].push((i, x + y));
    }

    let mut ret = vec![usize::MAX; n + 1];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if ret[now] != usize::MAX {
            continue;
        }
        ret[now] = nd;

        for &(to, d) in &t[now] {
            if ret[to] == usize::MAX {
                nt.push(Reverse((nd + d, to)));
            }
        }
    }

    println!("{}", ret.into_iter().skip(1).take(n - 1).join(" "));
}
