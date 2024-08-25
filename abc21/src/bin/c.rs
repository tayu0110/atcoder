use std::collections::VecDeque;

use proconio::*;

const MOD: usize = 1000000007;

fn main() {
    input! {n: usize, a: usize, b: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut dist = vec![usize::MAX; n];
    dist[a - 1] = 0;
    let mut res = vec![0; n];
    res[a - 1] = 1;
    let mut nt = VecDeque::new();
    nt.push_back(a - 1);
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[now] + 1 <= dist[to] {
                if dist[now] + 1 < dist[to] {
                    nt.push_back(to);
                }
                dist[to] = dist[now] + 1;
                res[to] += res[now];
                res[to] %= MOD;
            }
        }
    }

    println!("{}", res[b - 1]);
}
