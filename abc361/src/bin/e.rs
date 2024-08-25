use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, a: [(usize, usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for &(a, b, c) in &a {
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    let res = a.iter().map(|v| v.2).sum::<usize>() * 2;

    let mut root = 0;
    let mut dist = vec![usize::MAX; n];
    for _ in 0..2 {
        dist.fill(usize::MAX);
        let mut nt = VecDeque::new();
        nt.push_back((root, 0));
        while let Some((now, d)) = nt.pop_front() {
            dist[now] = d;

            for &(to, c) in &t[now] {
                if dist[to] == usize::MAX {
                    nt.push_back((to, d + c));
                }
            }
        }

        let &max = dist.iter().max().unwrap();
        root = dist.iter().position(|&c| c == max).unwrap();
    }

    println!("{}", res - dist.iter().max().unwrap());
}
