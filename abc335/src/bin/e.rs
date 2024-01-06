use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n], e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let mut res = vec![usize::MAX; n];
    let mut nt = BinaryHeap::new();
    nt.push((Reverse(a[0]), 1, 0));
    while let Some((_, nd, now)) = nt.pop() {
        if res[now] != usize::MAX {
            continue;
        }
        res[now] = nd;

        for &to in &t[now] {
            if a[now] > a[to] {
                continue;
            }

            if res[to] != usize::MAX {
                continue;
            }

            nt.push((Reverse(a[to]), nd + (a[now] != a[to]) as usize, to));
        }
    }

    // eprintln!("res: {res:?}");

    if res[n - 1] == usize::MAX {
        println!("0");
    } else {
        println!("{}", res[n - 1]);
    }
}
