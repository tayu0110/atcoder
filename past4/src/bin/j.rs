use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: usize, y: usize, z: usize, s: marker::Bytes}

    // A -> n   -> B
    // A -> n+1 -> C
    // B -> n+2 -> A
    // B -> n+3 -> C
    // C -> n+4 -> A
    // C -> n+5 -> B
    let mut t = vec![vec![]; n + 6];
    for _ in 0..m {
        input! {a: usize, b: usize, c: usize}
        t[a - 1].push((b - 1, c));
        t[b - 1].push((a - 1, c));
    }

    for (i, s) in s.into_iter().enumerate() {
        match s {
            b'A' => {
                t[n].push((i, x));
                t[n + 1].push((i, y));
                t[i].push((n + 2, 0));
                t[i].push((n + 4, 0));
            }
            b'B' => {
                t[n + 2].push((i, x));
                t[n + 3].push((i, z));
                t[i].push((n, 0));
                t[i].push((n + 5, 0));
            }
            b'C' => {
                t[n + 4].push((i, y));
                t[n + 5].push((i, z));
                t[i].push((n + 1, 0));
                t[i].push((n + 3, 0));
            }
            _ => {}
        }
    }

    let mut res = vec![usize::MAX; n + 6];
    let mut nt = BinaryHeap::new();
    nt.push(Reverse((0, 0)));
    while let Some(Reverse((nd, now))) = nt.pop() {
        if res[now] != usize::MAX {
            continue;
        }
        res[now] = nd;
        for &(to, c) in &t[now] {
            if res[to] == usize::MAX {
                nt.push(Reverse((nd + c, to)));
            }
        }
    }

    println!("{}", res[n - 1])
}
