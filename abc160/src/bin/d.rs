use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, x: usize, y: usize}

    let mut t = vec![vec![]; n];
    for i in 0..n - 1 {
        t[i].push(i + 1);
        t[i + 1].push(i);
    }
    t[x - 1].push(y - 1);
    t[y - 1].push(x - 1);

    let mut res = vec![0; n];
    for i in 0..n {
        let mut dist = vec![std::usize::MAX; n];
        dist[i] = 0;
        let mut nt = VecDeque::new();
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            for &to in &t[now] {
                if dist[to] == std::usize::MAX {
                    dist[to] = dist[now] + 1;
                    nt.push_back(to);
                }
            }
        }

        for j in i + 1..n {
            res[dist[j]] += 1;
        }
    }

    println!("{}", res[1..].iter().join("\n"))
}
