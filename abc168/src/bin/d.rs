#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0));
    while let Some((now, nd)) = nt.pop_front() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;

        for &to in &t[now] {
            nt.push_back((to, nd + 1));
        }
    }

    let mut res = vec![];
    for i in (1..n).rev() {
        for &from in &t[i] {
            if dist[from] + 1 == dist[i] {
                res.push(from);
                break;
            }
        }
    }

    if res.len() != n - 1 {
        println!("No");
        return;
    }

    res.reverse();
    println!("Yes");
    println!("{}", res.iter().map(|v| v + 1).join("\n"));
}
