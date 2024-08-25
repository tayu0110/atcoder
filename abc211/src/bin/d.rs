#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    const MOD: usize = 1_000_000_007;
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut res = vec![0; n];
    let mut reached = vec![false; n];
    res[0] = 1;
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0));
    while let Some((nd, now)) = nt.pop_front() {
        if reached[now] {
            continue;
        }
        dist[now] = nd;
        reached[now] = true;

        for &to in &t[now] {
            if dist[to] == nd+1 {
                res[to] += res[now];
                res[to] %= MOD;
            } else if dist[to] > nd+1 {
                res[to] = res[now];
                dist[to] = nd+1;
                nt.push_back((nd+1, to));
            }
        }
    }

    println!("{}", res[n-1]);
}
