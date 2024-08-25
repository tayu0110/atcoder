use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn path(n: usize, s: usize, g: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut dist = vec![usize::MAX; n];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, s)));
    while let Some(Reverse((d, now))) = nt.pop() {
        if dist[now] != usize::MAX {
            continue;
        }
        dist[now] = d;

        for &(to, c) in &g[now] {
            if dist[to] == usize::MAX {
                nt.push(Reverse((d + c, to)));
            }
        }
    }
    dist
}

fn main() {
    input! {n: usize, m: usize, s: usize, t: usize, l: usize, k: usize, e: [(usize, usize, usize); m]}

    let mut g = vec![vec![]; n];
    for (a, b, c) in e {
        g[a - 1].push((b - 1, c));
        g[b - 1].push((a - 1, c));
    }

    let d = path(n, s - 1, &g);

    if d[t - 1] <= k {
        println!("{}", n * (n - 1) / 2);
        return;
    }

    let rd = path(n, t - 1, &g);

    let mut p = rd.clone();
    p.sort_unstable();

    let mut res = 0;
    for (i, &d) in d.iter().enumerate() {
        if d.saturating_add(l) > k {
            continue;
        }

        let rem = k - d - l;
        res += p.partition_point(|&p| p <= rem);
        if rd[i] <= rem {
            res -= 1;
        }
    }

    println!("{}", res);
}
