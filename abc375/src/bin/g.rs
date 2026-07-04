use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use cpgraph::FixedGraph;
use itertools::Itertools;
use proconio::*;

fn dijkstra(n: usize, start: usize, t: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut dist = vec![usize::MAX; n];
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
    dist
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for &(a, b, c) in &e {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let dist = dijkstra(n, 0, &t);
    let rdist = dijkstra(n, n - 1, &t);
    let shortest = dist[n - 1];

    // true: Yes, false: No
    let mut res = vec![true; m];
    let mut edges = vec![];
    for (i, &(a, b, c)) in e.iter().enumerate() {
        if dist[a - 1] + rdist[b - 1] + c != shortest && dist[b - 1] + rdist[a - 1] + c != shortest
        {
            res[i] = false;
        } else {
            edges.push((a - 1, b - 1, i));
        }
    }

    let graph = FixedGraph::<(), false>::from_edges(edges.iter().cloned().map(|v| (v.0, v.1, ())));
    let bridges = graph.bridges().collect::<HashSet<(usize, usize)>>();
    for (a, b, i) in edges {
        if !bridges.contains(&(a, b)) && !bridges.contains(&(b, a)) {
            res[i] = false;
        }
    }

    println!(
        "{}",
        res.into_iter()
            .map(|f| if f { "Yes" } else { "No" })
            .join("\n")
    );
}
