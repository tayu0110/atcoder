use std::{cmp::Reverse, collections::BinaryHeap, time::Instant};

use proconio::*;
use rand::{seq::SliceRandom, thread_rng};

fn main() {
    input! {n: usize, m: usize, k: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut rt = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push((v - 1, 0));
        rt[v - 1].push((u - 1, 0));
    }

    let mut rng = thread_rng();
    let mut nt = BinaryHeap::new();
    let mut max = 0;
    let tm = Instant::now();
    while tm.elapsed().as_millis() < 1980 {
        let mut t = t.clone();
        let mut rt = rt.clone();
        for i in 0..=k {
            nt.push(Reverse((0, 0)));
            let mut dist = vec![usize::MAX; n];
            while let Some(Reverse((nd, now))) = nt.pop() {
                if dist[now] <= nd {
                    continue;
                }
                dist[now] = nd;
                for &(to, cost) in &t[now] {
                    if dist[now] + cost < dist[to] {
                        nt.push(Reverse((nd + cost, to)));
                    }
                }
            }
            max = max.max(dist[n - 1]);
            if i == k {
                break;
            }

            let mut path = vec![];
            let mut now = n - 1;
            while now != 0 {
                for &(from, cost) in &rt[now] {
                    if dist[from] + cost == dist[now] {
                        if cost == 0 {
                            path.push((from, now));
                        }
                        now = from;
                    }
                }
            }

            if let Some(&(from, to)) = path.choose(&mut rng) {
                for i in 0..t[from].len() {
                    if t[from][i].0 == to {
                        t[from][i].1 = 1;
                        break;
                    }
                }
                for i in 0..rt[to].len() {
                    if rt[to][i].0 == from {
                        rt[to][i].1 = 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{max}")
}
