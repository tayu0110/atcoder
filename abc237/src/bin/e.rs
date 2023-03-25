#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, h: [i64; n], p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (mut u, mut v) in p {
        if h[u-1] < h[v-1] {
            std::mem::swap(&mut u, &mut v);
        }
        t[u-1].push((v-1, 0));
        t[v-1].push((u-1, h[u-1] - h[v-1]));
    }

    let mut dist = vec![std::i64::MAX; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0usize)));
    while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
        if dist[now] <= nd {
            continue;
        }
        dist[now] = nd;
        for (to, w) in &t[now] {
            if dist[now] + *w < dist[*to] {
                nt.push(std::cmp::Reverse((dist[now] + *w, *to)));
            }
        }
    }

    // eprintln!("dist: {:?}", dist);
    dist.iter_mut().enumerate().for_each(|(i, d)| *d += h[i] - h[0]);
    // eprintln!("dist: {:?}", dist);
    println!("{}", -dist.iter().min().unwrap());
}
