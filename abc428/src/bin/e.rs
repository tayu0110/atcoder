use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
    }

    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    let mut nt = VecDeque::new();
    nt.push_back(0);
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[to] == usize::MAX {
                dist[to] = dist[now] + 1;
                nt.push_back(to);
            }
        }
    }

    let &max = dist.iter().max().unwrap();
    let pos = dist.iter().rposition(|c| *c == max).unwrap();

    let mut dist = vec![usize::MAX; n];
    dist[pos] = 0;
    let mut nt = VecDeque::new();
    nt.push_back(pos);
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist[to] == usize::MAX {
                dist[to] = dist[now] + 1;
                nt.push_back(to);
            }
        }
    }

    let &max = dist.iter().max().unwrap();
    let rpos = dist.iter().rposition(|c| *c == max).unwrap();

    let mut dist2 = vec![usize::MAX; n];
    dist2[rpos] = 0;
    let mut nt = VecDeque::new();
    nt.push_back(rpos);
    while let Some(now) = nt.pop_front() {
        for &to in &t[now] {
            if dist2[to] == usize::MAX {
                dist2[to] = dist2[now] + 1;
                nt.push_back(to);
            }
        }
    }

    let mut ret = vec![];
    for i in 0..n {
        if dist[i] > dist2[i] || (dist[i] == dist2[i] && pos > rpos) {
            ret.push(pos + 1);
        } else {
            ret.push(rpos + 1);
        }
    }

    println!("{}", ret.iter().join("\n"))
}
