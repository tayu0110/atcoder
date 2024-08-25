use std::{cmp::Reverse, collections::BinaryHeap};

use iolib::*;

fn main() {
    scan!(n: usize, m: usize, e: [(usize, usize, usize); m]);

    let mut t = vec![vec![]; n];
    let mut rev = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        rev[v - 1].push((u - 1, w));
    }

    let mut d = vec![usize::MAX; n];
    let mut rd = vec![usize::MAX; n];
    for (start, dist, t) in [(0, &mut d, &t), (n - 1, &mut rd, &rev)] {
        let mut nt = BinaryHeap::new();
        nt.push(Reverse((0, start)));
        while let Some(Reverse((nd, now))) = nt.pop() {
            if dist[now] != usize::MAX {
                continue;
            }
            dist[now] = nd;
            for &(to, w) in &t[now] {
                if dist[to] == usize::MAX {
                    nt.push(Reverse((nd + w, to)));
                }
            }
        }
    }

    putitln!(d
        .into_iter()
        .zip(rd)
        .map(|(d, rd)| d.saturating_add(rd) as i64));
}
