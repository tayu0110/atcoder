use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, h: [u32; n], c: [usize; k]}

    let mut t = vec![vec![]; n];
    for _ in 0..m {
        input! {a: usize, b: usize}
        if h[a - 1] < h[b - 1] {
            t[a - 1].push(b - 1);
        } else {
            t[b - 1].push(a - 1);
        }
    }

    let mut nt = VecDeque::new();
    for c in c {
        nt.push_back((c - 1, 0));
    }

    let mut res = vec![usize::MAX; n];
    while let Some((now, nd)) = nt.pop_front() {
        if res[now] != usize::MAX {
            continue;
        }

        res[now] = nd;
        for &to in &t[now] {
            if res[to] == usize::MAX {
                nt.push_back((to, nd + 1));
            }
        }
    }

    println!("{}", res.iter().map(|&r| r as i64).join("\n"))
}
