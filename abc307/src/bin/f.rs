use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize, usize); m], k: usize, a: [usize; k], d: usize, x: [usize; d]}

    let mut t = vec![vec![]; n];
    for (u, v, w) in e {
        t[u - 1].push((v - 1, w));
        t[v - 1].push((u - 1, w));
    }

    let mut nt = BinaryHeap::new();
    let mut res = vec![std::usize::MAX; n];
    for a in a {
        res[a - 1] = 0;
        for &(to, cost) in &t[a - 1] {
            nt.push(Reverse((cost, to)));
        }
    }

    for (i, x) in x.into_iter().enumerate() {
        let mut next = vec![];
        while let Some(Reverse((d, now))) = nt.pop() {
            if res[now] != std::usize::MAX {
                continue;
            }
            if d <= x {
                res[now] = i + 1;
                for &(to, cost) in &t[now] {
                    if res[to] == std::usize::MAX {
                        if d + cost <= x {
                            nt.push(Reverse((d + cost, to)));
                        }
                        next.push((cost, to));
                    }
                }
            } else {
                nt.push(Reverse((d, now)));
                break;
            }
        }

        for (c, next) in next {
            if res[next] == std::usize::MAX {
                nt.push(Reverse((c, next)));
            }
        }
    }

    for i in 0..n {
        if res[i] == std::usize::MAX {
            println!("-1")
        } else {
            println!("{}", res[i])
        }
    }
}
