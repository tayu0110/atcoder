#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(usize, usize, usize); m], s: [usize; k]};

    let buttons = s.into_iter().map(|c| c - 1).collect::<std::collections::HashSet<_>>();

    let mut t = vec![vec![]; n];
    for (u, v, a) in p {
        t[u-1].push((v-1, a));
        t[v-1].push((u-1, a));
    }

    let mut dist = vec![vec![std::usize::MAX; 2]; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0, 0)));

    while let Some(std::cmp::Reverse((nd, now, pushed))) = nt.pop() {
        if dist[now][pushed % 2] != std::usize::MAX {
            continue;
        }
        dist[now][pushed % 2] = nd;

        let has_button = buttons.contains(&now);

        for &(to, a) in &t[now] {
            if pushed % 2 != a {
                nt.push(std::cmp::Reverse((nd + 1, to, pushed)));
            } else if has_button {
                nt.push(std::cmp::Reverse((nd + 1, to, pushed + 1)));
            }
        }
    }

    let res = std::cmp::min(dist[n-1][0], dist[n-1][1]);
    if res == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }

}
