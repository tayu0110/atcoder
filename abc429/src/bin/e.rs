use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], s: marker::Bytes}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut ret = vec![[[usize::MAX; 2]; 2]; n];
    let mut nt = (0..n)
        .filter_map(|i| (s[i] == b'S').then_some((i, i, 0)))
        .collect::<VecDeque<_>>();
    while let Some((start, now, nd)) = nt.pop_front() {
        for &to in &t[now] {
            if ret[to][0][0] != start && ret[to][1][0] != start && ret[to][1][1] == usize::MAX {
                if ret[to][0][0] == usize::MAX {
                    ret[to][0] = [start, nd + 1];
                } else {
                    ret[to][1] = [start, nd + 1];
                }
                nt.push_back((start, to, nd + 1));
            }
        }
    }

    println!(
        "{}",
        (0..n)
            .filter_map(|i| (s[i] == b'D').then_some(ret[i][0][1] + ret[i][1][1]))
            .join("\n")
    )
}
