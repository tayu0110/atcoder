use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut t = vec![vec![]; n];
    for _ in 0..m {
        input! {u: usize, v: usize, w: usize}
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, w));
    }

    let mut d = vec![usize::MAX; n];
    let mut s = vec![0; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, usize::MAX - a[0], 0)));
    while let Some(Reverse((nd, nc, now))) = nt.pop() {
        if d[now] != usize::MAX {
            continue;
        }
        d[now] = nd;
        s[now] = usize::MAX - nc;

        for &(to, w) in &t[now] {
            if d[to] == usize::MAX {
                nt.push(Reverse((nd + w, nc - a[to], to)));
            }
        }
    }

    println!("{}", s[n - 1]);
}
