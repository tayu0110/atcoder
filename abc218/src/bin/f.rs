#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut rt = vec![vec![]; n];
    for &(u, v) in &p {
        t[u-1].push(v-1);
        rt[v-1].push(u-1);
    }

    let mut dist = vec![std::usize::MAX; n];
    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, 0)));
    while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
        if dist[now] != std::usize::MAX {
            continue;
        }
        dist[now] = nd;

        for to in &t[now] {
            if dist[*to] == std::usize::MAX {
                nt.push(std::cmp::Reverse((nd+1, *to)));
            }
        }
    }

    let mut route = std::collections::HashSet::new();
    {
        let mut now = n-1;
        while dist[now] != std::usize::MAX && now != 0 {
            'base: for &to in &rt[now] {
                if dist[to] == dist[now]-1 {
                    for i in 0..m {
                        if p[i].0-1 == to && p[i].1-1 == now {
                            route.insert(i);
                            now = to;
                            break 'base;
                        }
                    }
                }
            }
        }

        if now != 0 {
            println!("{}", (0..m).map(|_| -1).join(" "));
            return;
        }
    }

    for i in 0..m {
        if route.contains(&i) {
            let bad = p[i];
            let mut nt = std::collections::BinaryHeap::new();
            nt.push(std::cmp::Reverse((0, 0)));
            let mut dist = vec![std::usize::MAX; n];
            while let Some(std::cmp::Reverse((nd, now))) = nt.pop() {
                if dist[now] != std::usize::MAX {
                    continue;
                }
                dist[now] = nd;

                for &to in &t[now] {
                    if (now+1, to+1) == bad {
                        continue;
                    }

                    if dist[to] == std::usize::MAX {
                        nt.push(std::cmp::Reverse((nd+1, to)));
                    }
                }
            }

            if dist[n-1] == std::usize::MAX {
                println!("-1");
            } else {
                println!("{}", dist[n-1]);
            }
        } else {
            println!("{}", dist[n-1]);
        }
    }
}
