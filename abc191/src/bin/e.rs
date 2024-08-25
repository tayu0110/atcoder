#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn dijkstra(start: usize, t: &[Vec<(usize, i64)>]) -> i64 {
    let mut nt = std::collections::BinaryHeap::new();
    let n = t.len();
    let mut dist = vec![-1; n];
    nt.push(std::cmp::Reverse((0, start)));

    while let Some(std::cmp::Reverse((d, now))) = nt.pop() {
        if now == start && d > 0 {
            return d;
        }
        if dist[now] >= 0 {
            continue;
        }
        dist[now] = d;

        for (to, cost) in &t[now] {
            nt.push(std::cmp::Reverse((d + *cost, *to)));
        }
    }

    -1
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, i64); m]};

    let mut t = vec![vec![]; n];
    for (a, b, c) in p {
        t[a-1].push((b-1, c));
    }

    for start in 0..n {
        println!("{}", dijkstra(start, &t));
    }
}
