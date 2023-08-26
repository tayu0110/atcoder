use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], k: usize, q: [(usize, usize); k]}

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    // 0: undecied, 1: white
    let mut memo = vec![0; n];

    for &(p, d) in &q {
        let mut dist = vec![std::usize::MAX; n];
        let mut nt = VecDeque::new();
        nt.push_back((p - 1, 0));
        while let Some((now, nd)) = nt.pop_front() {
            if dist[now] != std::usize::MAX {
                continue;
            }
            dist[now] = nd;
            if nd == d {
                continue;
            }
            memo[now] = 1;

            for &to in &t[now] {
                if dist[to] == std::usize::MAX {
                    nt.push_back((to, nd + 1));
                }
            }
        }
    }

    for (p, d) in q {
        let mut bad = true;
        let mut dist = vec![std::usize::MAX; n];
        let mut nt = VecDeque::new();
        nt.push_back((p - 1, 0));
        while let Some((now, nd)) = nt.pop_front() {
            if dist[now] != std::usize::MAX {
                continue;
            }
            dist[now] = nd;
            if nd == d && memo[now] == 0 {
                bad = false;
            } else if nd < d && memo[now] == 0 {
                println!("No");
                return;
            }

            for &to in &t[now] {
                if dist[to] == std::usize::MAX {
                    nt.push_back((to, nd + 1));
                }
            }
        }

        if bad {
            println!("No");
            return;
        }
    }

    println!("Yes");
    println!("{}", memo.into_iter().map(|i| 1 - i).join(""))
}
