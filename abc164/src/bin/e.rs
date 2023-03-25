#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, s: usize, p: [(usize, usize, usize, usize); m], c: [(usize, usize); n]}

    let mut t = vec![vec![]; n];
    let mut amax = 0;
    for (u, v, a, b) in p {
        t[u-1].push((v-1, a, b));
        t[v-1].push((u-1, a, b));
        amax = std::cmp::max(amax, a);
    }
    amax *= n - 1;

    let mut dist = vec![vec![std::usize::MAX; amax+1]; n];
    let mut check = vec![vec![false; amax+1]; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, std::cmp::min(amax, s), 0)));

    while let Some(std::cmp::Reverse((nd, coin, now))) = nt.pop() {
        if check[now][coin] {
            continue;
        }
        dist[now][coin] = nd;
        check[now][coin] = true;

        let (c, d) = c[now];

        for &(to, a, b) in &t[now] {
            let mut nnd = nd + b;
            let mut nc = coin;
            if a > coin {
                nnd += (a - coin + c - 1) / c * d;
                nc += (a - coin + c - 1) / c * c;
                nc = std::cmp::min(amax, nc);
            }
            if nnd < dist[to][nc - a] {
                dist[to][nc - a] = nnd;
                nt.push(std::cmp::Reverse((nnd, nc - a, to)));
            }
            while nc < amax {
                nnd = nnd + d;
                nc = std::cmp::min(amax, nc + c);

                if nnd < dist[to][nc - a] {
                    dist[to][nc - a] = nnd;
                    nt.push(std::cmp::Reverse((nnd, nc - a, to)));
                }
            }
        }
    }

    for i in 1..n {
        let res = dist[i].iter().cloned().min().unwrap();
        println!("{}", res);
    }
}
