#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, q: usize, p: [(usize, usize); n-1], q: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0));
    while let Some((nd, now)) = nt.pop_front() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;
        for to in &t[now] {
            if dist[*to] == std::usize::MAX {
                nt.push_back((nd+1, *to));
            }
        }
    }

    for (c, d) in q {
        if (dist[c-1] + dist[d-1]) % 2 == 0 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
