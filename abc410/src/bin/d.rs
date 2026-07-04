use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b, w) in e {
        t[a - 1].push((b - 1, w));
    }

    let mut memo = vec![vec![false; 1 << 10]; n];
    memo[0][0] = true;
    let mut nt = VecDeque::new();
    nt.push_back((0, 0));
    while let Some((nd, now)) = nt.pop_front() {
        for &(to, w) in &t[now] {
            let next = nd ^ w;
            if !memo[to][next] {
                memo[to][next] = true;
                nt.push_back((next, to));
            }
        }
    }

    for i in 0..1 << 10 {
        if memo[n - 1][i] {
            println!("{i}");
            return;
        }
    }

    println!("-1")
}
