use itertools::Itertools;
use proconio::*;
use std::collections::BinaryHeap;

fn main() {
    input! {n: usize, m: usize, k: usize, e: [(usize, usize); m], p: [(usize, usize); k]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut nt = BinaryHeap::new();
    for (p, h) in p {
        nt.push((h, p - 1));
    }

    let mut reached = vec![false; n];
    while let Some((h, now)) = nt.pop() {
        if reached[now] {
            continue;
        }
        reached[now] = true;

        if h == 0 {
            continue;
        }

        for &to in &t[now] {
            if !reached[to] {
                nt.push((h - 1, to));
            }
        }
    }

    let res = reached
        .into_iter()
        .enumerate()
        .filter(|&(_, f)| f)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    println!("{}", res.len());
    println!("{}", res.into_iter().join(" "))
}
