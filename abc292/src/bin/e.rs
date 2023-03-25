use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u - 1].push(v - 1);
    }

    let mut res = 0;
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

        res += dist
            .into_iter()
            .filter(|v| *v > 1 && *v != std::usize::MAX)
            .count() as usize;
    }

    println!("{}", res);
}
